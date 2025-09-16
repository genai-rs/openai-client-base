#!/usr/bin/env python3
"""
Fix untagged anyOf unions that OpenAPI Generator incorrectly generates as empty structs
or flattened structs. This converts them to proper Rust enums using serde's untagged feature.

This script automatically detects untagged unions from the OpenAPI spec and fixes them.
"""

import os
import sys
from utils import (
    convert_to_rust_type_name,
    snake_to_pascal,
    pascal_to_snake,
    load_spec,
    sanitize_variant_name
)

def _merge_simple_string_enums(target, new_entries):
    for name, info in new_entries.items():
        if name in target:
            existing = target[name]
            if existing.get('default') is None and info.get('default') is not None:
                existing['default'] = info['default']
            # Prefer existing values unless new info provides a larger set
            if len(info.get('values', [])) > len(existing.get('values', [])):
                existing['values'] = info['values']
        else:
            target[name] = {
                'values': list(info.get('values', [])),
                'default': info.get('default'),
            }


def detect_untagged_unions(spec):
    """Detect all untagged anyOf/oneOf types in the spec"""
    untagged_unions = {}
    simple_string_enums = {}
    
    schemas = spec.get('components', {}).get('schemas', {})
    
    for schema_name, schema_def in schemas.items():
        # Skip if it's a reference
        if '$ref' in schema_def:
            continue
            
        # Check properties for anyOf/oneOf without discriminator
        if isinstance(schema_def, dict) and schema_def.get('type') == 'object':
            props = schema_def.get('properties', {})
            for prop_name, prop_def in props.items():
                if 'anyOf' in prop_def or 'oneOf' in prop_def:
                    # Check if it's tagged (has discriminator)
                    if 'discriminator' not in prop_def:
                        # Convert property name to PascalCase properly
                        prop_pascal = ''.join(word.capitalize() for word in prop_name.replace('_', ' ').split())
                        type_name = f"{schema_name}{prop_pascal}"
                        variants, enum_types = analyze_union_variants(
                            prop_def.get('anyOf') or prop_def.get('oneOf'),
                            schemas,
                            type_name
                        )
                        if variants:
                            untagged_unions[type_name] = variants
                            _merge_simple_string_enums(simple_string_enums, enum_types)
        
        # Check if the schema itself is an untagged union
        if 'anyOf' in schema_def or 'oneOf' in schema_def:
            if 'discriminator' not in schema_def:
                variants, enum_types = analyze_union_variants(
                    schema_def.get('anyOf') or schema_def.get('oneOf'),
                    schemas,
                    schema_name
                )
                if variants:
                    untagged_unions[schema_name] = variants
                    _merge_simple_string_enums(simple_string_enums, enum_types)
    
    return untagged_unions, simple_string_enums


def _get_ref_name(node):
    if isinstance(node, dict) and '$ref' in node and isinstance(node['$ref'], str):
        ref = node['$ref']
        if ref.startswith('#/components/schemas/'):
            return ref.split('/')[-1]
    return None


def detect_mixed_tagged_unions(spec):
    """Detect unions that declare a discriminator but mix string (or string enum) with object refs.
    Return mapping of type_name -> variants list [(VariantName, VariantType)].
    """
    schemas = spec.get('components', {}).get('schemas', {})
    results = {}

    def resolve_schema(ref_name):
        return schemas.get(ref_name, {})

    def analyze_union_items(items):
        has_string = False
        variants = []
        for it in items:
            if not isinstance(it, dict):
                continue
            # $ref
            ref = _get_ref_name(it)
            if ref:
                ref_schema = resolve_schema(ref)
                # string enum schema
                if ref_schema.get('type') == 'string':
                    # Use a generic variant name
                    variants.append(('Options', f'models::{convert_to_rust_type_name(ref)}'))
                    has_string = True
                else:
                    # object or other types
                    variants.append((sanitize_variant_name(ref), f'models::{convert_to_rust_type_name(ref)}'))
                continue
            # inline string
            if it.get('type') == 'string':
                values = it.get('enum')
                if values:
                    # Synthesize a simple enum name from title or fallback
                    title = it.get('title', 'OptionsEnum')
                    enum_name = sanitize_variant_name(title) + 'Enum'
                    variants.append(('Options', enum_name))
                else:
                    variants.append(('Text', 'String'))
                has_string = True
                continue
        return has_string, variants

    def add_result(type_name, variants):
        # Ensure at least two variants
        if len(variants) >= 2:
            results[type_name] = variants

    # Top-level schemas
    for schema_name, schema_def in schemas.items():
        if not isinstance(schema_def, dict):
            continue
        if 'discriminator' in schema_def and ('anyOf' in schema_def or 'oneOf' in schema_def):
            items = schema_def.get('anyOf') or schema_def.get('oneOf') or []
            has_string, variants = analyze_union_items(items)
            # Mixed if includes string and object refs
            if has_string and any(vt.startswith('models::') for _, vt in variants):
                add_result(schema_name, variants)

        # Properties
        if schema_def.get('type') == 'object':
            for prop_name, prop_def in (schema_def.get('properties') or {}).items():
                if not isinstance(prop_def, dict):
                    continue
                if 'discriminator' in prop_def and ('anyOf' in prop_def or 'oneOf' in prop_def):
                    items = prop_def.get('anyOf') or prop_def.get('oneOf') or []
                    has_string, variants = analyze_union_items(items)
                    if has_string and any(vt.startswith('models::') for _, vt in variants):
                        type_name = f"{schema_name}{''.join(word.capitalize() for word in prop_name.replace('_',' ').split())}"
                        add_result(type_name, variants)

    return results

def analyze_union_variants(union_items, schemas, context_name=None, simple_string_enums=None):
    """Analyze anyOf/oneOf items to determine variant types"""
    variants = []
    enum_types = {}  # Local tracking of enum types
    
    for item in union_items:
        if not isinstance(item, dict):
            continue

        # Flatten nested anyOf/oneOf structures before handling concrete nodes
        nested = item.get('anyOf') or item.get('oneOf')
        if nested:
            nested_variants, nested_enum_types = analyze_union_variants(nested, schemas, context_name)
            for variant in nested_variants:
                if variant not in variants:
                    variants.append(variant)
            enum_types.update(nested_enum_types)
            continue
            
        # Handle string type
        if item.get('type') == 'string':
            title = item.get('title', 'Text')
            # Check if it's an enum
            if 'enum' in item:
                # This is a string enum, create a type name for it
                # Ensure unique variant names
                variant_name = sanitize_variant_name(title)
                # If the variant name would conflict, add a suffix
                existing_names = [v[0] for v in variants]
                if variant_name in existing_names or variant_name == 'Text':
                    variant_name = variant_name + 'Variant'
                base_name = convert_to_rust_type_name(context_name) if context_name else ''
                enum_name = f"{base_name}{variant_name}Enum" if base_name else variant_name + 'Enum'
                variants.append((variant_name, enum_name))
                # Store the enum values for later creation
                enum_types[enum_name] = {
                    'values': item['enum'],
                    'default': item.get('default'),
                }
            else:
                variant_name = sanitize_variant_name(title)
                # Avoid duplicate 'Text' variants
                existing_names = [v[0] for v in variants]
                if variant_name in existing_names:
                    variant_name = variant_name + 'String'
                variants.append((variant_name, 'String'))
        
        # Handle array type
        elif item.get('type') == 'array':
            title = item.get('title', 'Array')
            items_def = item.get('items', {})
            
            if '$ref' in items_def:
                ref_type = items_def['$ref'].split('/')[-1]
                # Check if the referenced type actually generates a file or if it's a nested reference
                # For ChatCompletionRequestSystemMessageContentPart, it's actually ChatCompletionRequestMessageContentPartText
                if ref_type == 'ChatCompletionRequestSystemMessageContentPart':
                    # This is a special case - the type is an anyOf with a single ref
                    ref_type = 'ChatCompletionRequestMessageContentPartText'
                elif ref_type == 'ChatCompletionRequestToolMessageContentPart':
                    ref_type = 'ChatCompletionRequestMessageContentPartText'
                elif ref_type == 'ChatCompletionRequestDeveloperMessageContentPart':
                    ref_type = 'ChatCompletionRequestMessageContentPartText'
                variants.append((sanitize_variant_name(title), f'Vec<models::{ref_type}>'))
            elif items_def.get('type') == 'string':
                variants.append(('ArrayOfStrings', 'Vec<String>'))
            elif items_def.get('type') == 'integer':
                variants.append(('ArrayOfIntegers', 'Vec<i32>'))
            elif items_def.get('type') == 'array':
                inner = items_def.get('items', {})
                if inner.get('type') == 'integer':
                    variants.append(('ArrayOfIntegerArrays', 'Vec<Vec<i32>>'))
            else:
                variants.append((sanitize_variant_name(title), 'Vec<serde_json::Value>'))
        
        # Handle object type with specific structure
        elif item.get('type') == 'object' or '$ref' in item:
            if '$ref' in item:
                ref_name = item['$ref'].split('/')[-1]
                title = item.get('title', ref_name)
                
                # Check if the referenced schema is actually an array type
                if ref_name in schemas and schemas[ref_name].get('type') == 'array':
                    # This is a reference to an array schema, expand it inline
                    array_items = schemas[ref_name].get('items', {})
                    if '$ref' in array_items:
                        inner_type = array_items['$ref'].split('/')[-1]
                        variants.append((sanitize_variant_name(title), f'Vec<models::{inner_type}>'))
                    else:
                        # Unknown array item type
                        variants.append((sanitize_variant_name(title), 'Vec<serde_json::Value>'))
                else:
                    # Regular object reference - apply proper Rust type name conversion
                    ref_name = convert_to_rust_type_name(ref_name)
                    variants.append((sanitize_variant_name(title), f'models::{ref_name}'))
            else:
                # Check if it has specific properties that identify it
                props = item.get('properties', {})
                if 'type' in props or 'function' in props or 'custom' in props:
                    # This is likely a named tool choice or similar
                    title = item.get('title', 'Named')
                    variants.append((sanitize_variant_name(title), 'models::ChatCompletionNamedToolChoice'))
        
        # Handle null type
        elif item.get('type') == 'null':
            variants.append(('Null', '()'))
    
    return variants, enum_types

def find_enum_values(union_items):
    """Find enum values from union items"""
    for item in union_items:
        if isinstance(item, dict) and item.get('type') == 'string' and 'enum' in item:
            return item['enum']
    return None

def create_untagged_enum(name, variants):
    """Generate an untagged enum for anyOf unions"""
    lines = [
        f"/// {name} - Untagged union type",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(untagged)]",
        f"pub enum {name} {{",
    ]
    
    for variant_name, variant_type in variants:
        if variant_type == '()':
            lines.append(f"    {variant_name},")
        else:
            lines.append(f"    {variant_name}({variant_type}),")
    
    lines.append("}")
    lines.append("")
    
    # Add Default implementation only for simple types that we know support it
    first_variant_name, first_variant_type = variants[0] if variants else (None, None)
    
    if first_variant_type in ['String', '()', 'Vec<String>', 'Vec<i32>']:
        lines.extend([
            f"impl Default for {name} {{",
            f"    fn default() -> Self {{",
        ])
        if first_variant_type == 'String':
            lines.append(f"        Self::{first_variant_name}(String::new())")
        elif first_variant_type == '()':
            lines.append(f"        Self::{first_variant_name}")
        elif first_variant_type.startswith('Vec<'):
            lines.append(f"        Self::{first_variant_name}(Vec::new())")
        lines.extend([
            "    }",
            "}",
            "",
        ])
    
    # Add convenience methods for common patterns
    if len(variants) >= 2 and variants[0][1] == 'String':
        lines.extend([
            f"impl {name} {{",
            f"    pub fn new_text(text: String) -> Self {{",
            f"        Self::{variants[0][0]}(text)",
            f"    }}",
        ])
        
        # Add array constructor if second variant is an array
        for variant_name, variant_type in variants[1:]:
            if variant_type.startswith('Vec<'):
                inner_type = variant_type[4:-1]  # Extract type from Vec<...>
                method_name = f"new_{variant_name.lower()}"
                lines.extend([
                    f"    pub fn {method_name}(items: {variant_type}) -> Self {{",
                    f"        Self::{variant_name}(items)",
                    f"    }}",
                ])
        
        lines.extend([
            "}",
            "",
            f"impl From<String> for {name} {{",
            f"    fn from(s: String) -> Self {{",
            f"        Self::{variants[0][0]}(s)",
            f"    }}",
            f"}}",
            "",
            f"impl From<&str> for {name} {{",
            f"    fn from(s: &str) -> Self {{",
            f"        Self::{variants[0][0]}(s.to_string())",
            f"    }}",
            f"}}",
        ])
    
    # Add Display implementation that renders strings plainly and JSON serializes other variants
    lines.extend([
        f"impl std::fmt::Display for {name} {{",
        "    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {",
        "        match self {",
    ])

    for variant_name, variant_type in variants:
        if variant_type == '()':
            lines.append(f"            {name}::{variant_name} => write!(f, \"null\"),")
        elif variant_type == 'String':
            lines.append(f"            {name}::{variant_name}(value) => write!(f, \"{{}}\", value),")
        elif variant_type.endswith('Enum'):
            lines.append(f"            {name}::{variant_name}(value) => write!(f, \"{{}}\", value),")
        else:
            lines.extend([
                f"            {name}::{variant_name}(value) => match serde_json::to_string(value) {{",
                "                Ok(s) => write!(f, \"{}\", s),",
                "                Err(_) => Err(std::fmt::Error),",
                "            },",
            ])

    lines.extend([
        "        }",
        "    }",
        "}",
        "",
    ])

    lines.append("")
    return '\n'.join(lines)

def create_simple_string_enum(name, enum_info):
    """Generate a simple string enum"""
    values = enum_info['values']
    default_value = enum_info.get('default')
    lines = [
        f"/// {name} - String enum type",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"lowercase\")]",
        f"pub enum {name} {{",
    ]
    
    variant_value_pairs = []

    def to_variant(val: str) -> str:
        if val == 'none':
            return 'None'
        if val == 'auto':
            return 'Auto'
        if val == 'required':
            return 'Required'
        if val == 'json_object':
            return 'JsonObject'
        if val == 'json_schema':
            return 'JsonSchema'
        return ''.join(word.capitalize() for word in val.replace('-', '_').split('_'))

    for value in values:
        variant = to_variant(value)

        # Add serde rename if variant name differs from original value
        if variant.lower() != value:
            lines.append(f'    #[serde(rename = "{value}")]')
        lines.append(f"    {variant},")
        variant_value_pairs.append((variant, value))
    
    lines.extend([
        "}",
        "",
        f"impl Default for {name} {{",
        f"    fn default() -> Self {{",
    ])
    
    # Determine which variant should be default
    first_value = default_value if default_value in values else values[0]
    first_variant = None
    for variant, value in variant_value_pairs:
        if value == first_value:
            first_variant = variant
            break
    if first_variant is None and variant_value_pairs:
        first_variant = variant_value_pairs[0][0]

    lines.extend([
        f"        Self::{first_variant}",
        f"    }}",
        f"}}",
        "",
    ])

    # Implement Display so enums stringify to their wire representation
    lines.extend([
        f"impl std::fmt::Display for {name} {{",
        f"    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{",
        "        let value = match self {",
    ])

    for variant, value in variant_value_pairs:
        lines.append(f'            {name}::{variant} => "{value}",')

    lines.extend([
        "        };",
        "        write!(f, \"{}\", value)",
        "    }",
        "}",
        "",
    ])

    return '\n'.join(lines)


def fix_file(filepath, type_name, variants, simple_string_enums):
    """Fix a single model file"""
    print(f"  Fixing untagged union: {type_name}")
    
    # Apply proper Rust type name conversion
    rust_type_name = convert_to_rust_type_name(type_name)
    
    content = "use crate::models;\nuse serde::{Deserialize, Serialize};\n\n"
    content += create_untagged_enum(rust_type_name, variants)
    
    # Track which enum types we've already added
    added_enums = set()
    
    # Also create simple string enums if needed
    for variant_name, variant_type in variants:
        if variant_type in simple_string_enums and variant_type not in added_enums:
            content += create_simple_string_enum(variant_type, simple_string_enums[variant_type])
            added_enums.add(variant_type)
        elif variant_type.endswith('Enum') and variant_type not in added_enums:
            # Check if it's a referenced enum that should exist
            # For now, create a placeholder if we don't have the values
            if variant_type not in simple_string_enums:
                print(f"    Warning: Missing enum values for {variant_type}")
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    return True

def main():
    if len(sys.argv) < 2:
        print("Usage: fix_untagged_unions.py <root_dir> [spec_path]")
        sys.exit(1)
    
    root_dir = sys.argv[1]
    spec_path = sys.argv[2] if len(sys.argv) > 2 else os.path.join(root_dir, 'stainless.yaml')
    models_dir = os.path.join(root_dir, 'src', 'models')
    
    if not os.path.exists(models_dir):
        print(f"Models directory not found: {models_dir}")
        sys.exit(1)
    
    if not os.path.exists(spec_path):
        print(f"Error: OpenAPI spec not found: {spec_path}")
        print("The spec is required to detect untagged unions.")
        sys.exit(1)
    
    print(f"Loading OpenAPI spec from: {spec_path}")
    spec = load_spec(spec_path)
    print("Detecting untagged unions from spec...")
    untagged_unions, simple_string_enums = detect_untagged_unions(spec)
    # Add mixed tagged unions (string + object variants) to be treated as untagged
    mixed_unions = detect_mixed_tagged_unions(spec)
    for k, v in mixed_unions.items():
        if k not in untagged_unions:
            untagged_unions[k] = v
    
    # Add some known types that might not be detected correctly
    # These are message content types that need special handling
    # Note: we use the actual types that exist after generation
    known_content_types = {
        'ChatCompletionRequestSystemMessageContent': [
            ('Text', 'String'),
            ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
        ],
        'ChatCompletionRequestUserMessageContent': [
            ('Text', 'String'),
            ('Array', 'Vec<models::ChatCompletionRequestUserMessageContentPart>'),
        ],
        'ChatCompletionRequestAssistantMessageContent': [
            ('Text', 'String'),
            ('Array', 'Vec<models::ChatCompletionRequestAssistantMessageContentPart>'),
        ],
        'ChatCompletionRequestToolMessageContent': [
            ('Text', 'String'),
            ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
        ],
        'ChatCompletionRequestDeveloperMessageContent': [
            ('Text', 'String'),
            ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
        ],
    }
    
    # Merge known types with detected ones
    for name, variants in known_content_types.items():
        if name not in untagged_unions:
            untagged_unions[name] = variants
    
    print(f"Found {len(untagged_unions)} untagged union types to fix")
    print("Fixing untagged unions...")
    
    fixed_count = 0
    created_modules = set()
    
    # First, create files for simple string enums that might not exist
    for enum_name, enum_info in simple_string_enums.items():
        filename = pascal_to_snake(enum_name) + '.rs'
        filepath = os.path.join(models_dir, filename)
        new_content = "use serde::{Deserialize, Serialize};\n\n" + create_simple_string_enum(enum_name, enum_info)
        if not os.path.exists(filepath):
            print(f"  Creating new file: {filename}")
            with open(filepath, 'w') as f:
                f.write(new_content)
            created_modules.add(pascal_to_snake(enum_name))
            fixed_count += 1
        else:
            with open(filepath, 'r') as f:
                current_content = f.read()
            if current_content != new_content:
                print(f"  Updating file: {filename}")
                with open(filepath, 'w') as f:
                    f.write(new_content)
                fixed_count += 1
    
    # Now fix the untagged union types
    for type_name, variants in untagged_unions.items():
        filename = pascal_to_snake(type_name) + '.rs'
        filepath = os.path.join(models_dir, filename)
        
        if os.path.exists(filepath):
            if fix_file(filepath, type_name, variants, simple_string_enums):
                fixed_count += 1
        else:
            print(f"  Warning: File not found for {type_name}: {filepath}")

    # Targeted fix for mixed union ResponsePropertiesToolChoice:
    # This union includes a string enum (ToolChoiceOptions) and tagged object variants.
    # Note: ResponsePropertiesToolChoice will be handled by mixed union detection above.
    
    # Update mod.rs to include any new modules
    if created_modules:
        mod_rs_path = os.path.join(models_dir, 'mod.rs')
        if os.path.exists(mod_rs_path):
            with open(mod_rs_path, 'r') as f:
                content = f.read()
            
            lines = content.split('\n')
            for module_name in sorted(created_modules):
                pub_line = f"pub mod {module_name};"
                use_line = f"pub use self::{module_name}::{snake_to_pascal(module_name)};"
                
                if pub_line not in content:
                    # Find the right place to insert (alphabetically)
                    inserted = False
                    for i, line in enumerate(lines):
                        if line.startswith('pub mod ') and line > pub_line:
                            lines.insert(i, pub_line)
                            lines.insert(i + 1, use_line)
                            inserted = True
                            break
                    if not inserted:
                        # Add at the end of mod declarations
                        for i, line in enumerate(lines):
                            if not line.startswith('pub mod ') and i > 0:
                                lines.insert(i - 1, pub_line)
                                lines.insert(i, use_line)
                                break
                    
                    print(f"  Added module {module_name} to mod.rs")
            
            with open(mod_rs_path, 'w') as f:
                f.write('\n'.join(lines))
    
    print(f"Fixed {fixed_count} untagged union types")

if __name__ == '__main__':
    main()
