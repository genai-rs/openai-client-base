#!/usr/bin/env python3
"""
Fix empty enums in OpenAPI Generator output by adding proper variants.
This script handles tagged enums (with discriminator) that the generator leaves empty.
"""

import os
import re
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
from utils import (
    convert_to_rust_type_name,
    snake_to_pascal,
    pascal_to_snake,
    load_spec
)

def _get_ref_name(node: Dict[str, Any]) -> Optional[str]:
    if not isinstance(node, dict):
        return None
    ref = node.get('$ref')
    if isinstance(ref, str) and ref.startswith('#/components/schemas/'):
        return ref.split('/')[-1]
    return None


def _extract_discriminated_variants(
    union_schema: Dict[str, Any],
    schemas: Dict[str, Any],
    default_discriminator: str = 'type',
) -> Optional[Tuple[str, List[Dict[str, str]]]]:
    """Given a union schema that has anyOf/oneOf with a discriminator, return (property_name, variants)."""
    if not isinstance(union_schema, dict):
        return None
    if 'discriminator' not in union_schema:
        return None
    if 'anyOf' not in union_schema and 'oneOf' not in union_schema:
        return None

    discriminator = union_schema.get('discriminator', {})
    property_name = discriminator.get('propertyName', default_discriminator)
    items = union_schema.get('anyOf') or union_schema.get('oneOf') or []

    # If union mixes non-$ref items (e.g., string enum) with refs, skip here (handled by untagged union fixer)
    if any(not isinstance(it, dict) or ('$ref' not in it) for it in items):
        return None

    variant_info: List[Dict[str, str]] = []
    for it in items:
        ref_name = _get_ref_name(it)
        if not ref_name:
            continue
        ref_schema = schemas.get(ref_name, {})
        properties = ref_schema.get('properties', {})
        discriminator_value = None
        if property_name in properties:
            prop_def = properties[property_name]
            if isinstance(prop_def, dict):
                if 'const' in prop_def:
                    discriminator_value = prop_def['const']
                elif 'enum' in prop_def and isinstance(prop_def['enum'], list) and len(prop_def['enum']) == 1:
                    discriminator_value = prop_def['enum'][0]
        if not discriminator_value:
            # Fallback heuristic
            discriminator_value = ref_name.lower()
        variant_info.append({'schema': ref_name, 'discriminator_value': str(discriminator_value)})

    if not variant_info:
        return None
    return property_name, variant_info


def _derive_enum_names(schema_name: str, path_steps: List[Tuple[str, Optional[str]]], *, root_has_allof: bool = False) -> List[str]:
    """
    Derive likely Rust enum type names from a schema and traversal path.
    path_steps: list of (kind, name) where kind in {'property', 'array_items'}, name is property name for 'property'.
    """
    # Primary name follows openapi-generator pattern: Schema + PropsPascal + Inner for array items
    parts: List[str] = [schema_name]
    prop_only_parts: List[str] = []
    for kind, name in path_steps:
        if kind == 'property' and name:
            pas = snake_to_pascal(name)
            parts.append(pas)
            prop_only_parts.append(pas)
        elif kind == 'array_items':
            parts.append('Inner')
    candidates: List[str] = [''.join(parts)]

    # Special-case: array schema at root (e.g., ChatCompletionMessageToolCalls -> ...Inner)
    if len(path_steps) == 1 and path_steps[0][0] == 'array_items':
        candidates.append(f"{schema_name}Inner")

    # General allOf alias heuristic:
    # When a schema is composed via allOf into another schema, openapi-generator
    # often names derived property enums as `<Parent>AllOf<Prop>`.
    # We cannot know the parent here, but for the current schema we can emit an
    # `AllOf` alias variant that matches patterns like `Create*AllOf<Prop>`.
    # To avoid over-generation, only add this alias when the root schema has allOf.
    if root_has_allof and prop_only_parts:
        candidates.append(f"{schema_name}AllOf" + ''.join(prop_only_parts))

    # Also include a variant without trailing 'Inner' if any
    no_inner = ''.join([p for p in parts if p != 'Inner'])
    if no_inner not in candidates:
        candidates.append(no_inner)

    # Deduplicate
    uniq: List[str] = []
    for c in candidates:
        if c not in uniq:
            uniq.append(c)
    return uniq


def detect_empty_tagged_enums(spec_path: Path) -> Dict[str, Dict[str, Any]]:
    """
    Traverse the spec to detect discriminated unions at:
      - top-level schemas
      - property-level schemas
      - array items (including property arrays and top-level array schemas)
    Returns mapping from enum type name -> { property_name, variants }.
    """
    spec = load_spec(str(spec_path))
    schemas: Dict[str, Any] = spec.get('components', {}).get('schemas', {})

    result: Dict[str, Dict[str, Any]] = {}

    def resolve(node: Any) -> Any:
        ref = _get_ref_name(node) if isinstance(node, dict) else None
        if ref:
            return schemas.get(ref, {})
        return node

    def walk(schema_name: str, node: Any, path: List[Tuple[str, Optional[str]]], *, root_has_allof: bool = False):
        node = resolve(node)
        if not isinstance(node, dict):
            return

        # Union at current node
        extracted = _extract_discriminated_variants(node, schemas)
        if extracted:
            property_name, variants = extracted
            for enum_name in _derive_enum_names(schema_name, path, root_has_allof=root_has_allof):
                result[enum_name] = {
                    'property_name': property_name,
                    'variants': variants,
                }

        # allOf/oneOf/anyOf branches might contain nested structures
        for key in ('allOf', 'oneOf', 'anyOf'):
            if key in node and isinstance(node[key], list):
                for sub in node[key]:
                    walk(schema_name, sub, path, root_has_allof=root_has_allof)

        # object properties
        if node.get('type') == 'object':
            props = node.get('properties', {})
            if isinstance(props, dict):
                for prop_name, prop_schema in props.items():
                    walk(schema_name, prop_schema, path + [('property', prop_name)], root_has_allof=root_has_allof)

        # array items
        if node.get('type') == 'array' and 'items' in node:
            walk(schema_name, node['items'], path + [('array_items', None)], root_has_allof=root_has_allof)

    for name, schema in schemas.items():
        has_allof = isinstance(schema, dict) and 'allOf' in schema
        walk(name, schema, [], root_has_allof=has_allof)

    return result

def fix_case_sensitivity_in_enum(enum_file, enum_name, enum_info, content):
    """Fix case sensitivity issues in existing enum variants."""
    original_content = content
    fixed_any = False
    
    # Find all Box<models::TypeName> patterns and fix any with incorrect acronym casing
    # Also handle digits/underscores/dashes which may appear from spec names
    pattern = r'Box<models::([A-Za-z0-9_\-]+)>'
    
    def fix_type_name(match):
        type_name = match.group(1)
        # Sanitize invalid chars, then fix acronym casing
        sanitized = ''.join(ch for ch in type_name if ch.isalnum() or ch == '_')
        fixed_name = convert_to_rust_type_name(sanitized)
        if type_name != fixed_name:
            nonlocal fixed_any
            fixed_any = True
            print(f"  Fixed: Box<models::{type_name}> -> Box<models::{fixed_name}>")
            return f'Box<models::{fixed_name}>'
        return match.group(0)
    
    content = re.sub(pattern, fix_type_name, content)
    
    if fixed_any:
        with open(enum_file, 'w') as f:
            f.write(content)
        print(f"Fixed case sensitivity in {enum_name}")
        return True
    
    return False


def replace_enum_block(enum_name: str, content: str, new_enum_src: str) -> Optional[str]:
    """Replace the enum block for enum_name with new_enum_src. Returns new content or None if not found."""
    import re
    # Rough regex to capture the enum block including attributes
    pattern = rf'(#[^\n]*\n)*\s*pub enum {enum_name} \{{[\s\S]*?\}}'
    m = re.search(pattern, content, re.MULTILINE)
    if not m:
        return None
    start, end = m.span()
    return content[:start] + new_enum_src + content[end:]

def fix_empty_enum(models_dir, enum_name, enum_info):
    """Fix a single empty enum by adding proper variants."""
    # Convert to snake_case for the file name
    file_name = pascal_to_snake(enum_name) + '.rs'
    enum_file = models_dir / file_name
    
    if not enum_file.exists():
        print(f"Warning: File {enum_file} not found for enum {enum_name}")
        return False
    
    with open(enum_file, 'r') as f:
        content = f.read()
    
    # Check if enum is empty
    is_empty = bool(re.search(rf'pub enum {enum_name} \{{\s*\}}', content))
    if not is_empty:
        # If enum exists but has invalid identifiers (e.g., dashes), replace whole enum block
        enum_block_has_dash = f"pub enum {enum_name} {{" in content and '-' in content
        if enum_block_has_dash:
            # Build new enum source like in the empty case
            variants = []
            for variant in enum_info['variants']:
                schema_name = variant['schema']
                discriminator_value = variant['discriminator_value']
                sanitized_schema = ''.join(ch for ch in schema_name if ch.isalnum() or ch == '_')
                rust_type_name = convert_to_rust_type_name(sanitized_schema)
                vname = schema_name
                if schema_name.startswith(enum_name):
                    vname = schema_name[len(enum_name):]
                if not vname:
                    vname = schema_name
                sanitized_variant = ''.join(ch for ch in vname if ch.isalnum() or ch == '_')
                vname = convert_to_rust_type_name(sanitized_variant)
                variants.append(f'''    #[serde(rename = "{discriminator_value}")]\n    {vname}(Box<models::{rust_type_name}>),''')
            new_enum = f'''#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]\n#[serde(tag = "{enum_info['property_name']}")]\npub enum {enum_name} {{\n{chr(10).join(variants)}\n}}'''
            replaced = replace_enum_block(enum_name, content, new_enum)
            if replaced is not None:
                with open(enum_file, 'w') as f:
                    f.write(replaced)
                print(f"Rebuilt enum block for {enum_name} to sanitize identifiers")
                return True
        # Otherwise, even if not empty, fix case sensitivity / type names
        return fix_case_sensitivity_in_enum(enum_file, enum_name, enum_info, content)
    
    # Build the new enum with variants
    variants = []
    for variant in enum_info['variants']:
        schema_name = variant['schema']
        discriminator_value = variant['discriminator_value']
        
        # Convert schema name to proper Rust type name
        # Handle special cases like MCP -> Mcp
        # Sanitize schema/type names to valid Rust identifiers
        sanitized_schema = ''.join(ch for ch in schema_name if ch.isalnum() or ch == '_')
        rust_type_name = convert_to_rust_type_name(sanitized_schema)
        
        # Generate a variant name (remove common prefix if it exists)
        variant_name = schema_name
        if schema_name.startswith(enum_name):
            variant_name = schema_name[len(enum_name):]
        if not variant_name:
            variant_name = schema_name
        
        # Apply the same conversion rules to the variant name
        sanitized_variant = ''.join(ch for ch in variant_name if ch.isalnum() or ch == '_')
        variant_name = convert_to_rust_type_name(sanitized_variant)
        
        # Ensure variant name starts with uppercase
        if variant_name and variant_name[0].islower():
            variant_name = variant_name[0].upper() + variant_name[1:]
        
        variants.append(f'''    #[serde(rename = "{discriminator_value}")]
    {variant_name}(Box<models::{rust_type_name}>),''')
    
    if not variants:
        print(f"No variants found for {enum_name}")
        return False
    
    # Build the complete enum
    new_enum = f'''#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "{enum_info['property_name']}")]
pub enum {enum_name} {{
{chr(10).join(variants)}
}}'''
    
    # Replace the empty enum
    content = re.sub(
        rf'#\[derive\([^)]*\)\]\s*(?:#\[serde\([^)]*\)\]\s*)*pub enum {enum_name} \{{\s*\}}',
        new_enum,
        content,
        flags=re.MULTILINE | re.DOTALL
    )
    
    with open(enum_file, 'w') as f:
        f.write(content)
    
    print(f"Fixed {enum_name} enum with {len(variants)} variants")
    return True

def main():
    project_root = Path(__file__).parent.parent
    models_dir = project_root / "src" / "models"
    spec_path = project_root / "stainless.yaml"
    
    if not spec_path.exists():
        print(f"OpenAPI spec not found at {spec_path}")
        return
    
    if not models_dir.exists():
        print("No models directory found, skipping enum fixes")
        return
    
    print("Detecting empty tagged enums from OpenAPI spec (with traversal)...")
    empty_enums = detect_empty_tagged_enums(spec_path)
    
    if not empty_enums:
        print("No empty tagged enums detected")
        return
    
    print(f"Found {len(empty_enums)} potentially empty tagged enums")
    
    fixed_count = 0
    for enum_name, enum_info in empty_enums.items():
        if fix_empty_enum(models_dir, enum_name, enum_info):
            fixed_count += 1
    
    print(f"\nFixed {fixed_count} empty enums successfully!")

if __name__ == "__main__":
    main()
