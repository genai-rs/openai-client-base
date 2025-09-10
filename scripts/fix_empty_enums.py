#!/usr/bin/env python3
"""
Fix empty enums in OpenAPI Generator output by adding proper variants.
This script handles tagged enums (with discriminator) that the generator leaves empty.
"""

import os
import re
import yaml
from pathlib import Path

def snake_to_pascal(name):
    """Convert snake_case to PascalCase."""
    return ''.join(word.capitalize() for word in name.split('_'))

def convert_to_rust_type_name(name):
    """Convert type names to follow Rust naming conventions.
    Automatically detects and converts acronyms (consecutive uppercase letters) to PascalCase.
    Examples: MCP -> Mcp, HTTP -> Http, URL -> Url, API -> Api, MCPTool -> McpTool, etc.
    """
    result = []
    i = 0
    
    while i < len(name):
        # Check if we're at the start of an acronym (2+ consecutive uppercase letters)
        if i < len(name) - 1 and name[i].isupper() and name[i+1].isupper():
            # Found start of acronym, collect all uppercase letters
            acronym_start = i
            while i < len(name) and name[i].isupper():
                i += 1
            
            acronym = name[acronym_start:i]
            
            # Check what comes after the acronym
            if i < len(name) and name[i].islower():
                # Acronym is followed by lowercase (e.g., "MCPtool" or "HTTPServer")
                # The last uppercase letter is actually the start of the next word
                if len(acronym) > 1:
                    # Convert the acronym part (excluding last letter)
                    result.append(acronym[0] + acronym[1:-1].lower())
                    # Back up one position to reprocess the last uppercase letter
                    i -= 1
                else:
                    result.append(acronym)
            else:
                # Acronym is at the end or followed by another capital/nothing
                result.append(acronym[0] + acronym[1:].lower())
        else:
            # Regular character, just append it
            result.append(name[i])
            i += 1
    
    return ''.join(result)

def detect_empty_tagged_enums(spec_path):
    """
    Detect all schemas with discriminators that will result in empty enums.
    Returns a dict mapping enum names to their variant information.
    """
    with open(spec_path, 'r') as f:
        spec = yaml.safe_load(f)
    
    empty_enums = {}
    schemas = spec.get('components', {}).get('schemas', {})
    
    for schema_name, schema_def in schemas.items():
        # Check if this schema uses anyOf/oneOf with a discriminator
        if 'discriminator' in schema_def and ('anyOf' in schema_def or 'oneOf' in schema_def):
            discriminator = schema_def['discriminator']
            property_name = discriminator.get('propertyName', 'type')
            
            # Get the variants
            variants = schema_def.get('anyOf', schema_def.get('oneOf', []))
            variant_info = []
            
            for variant in variants:
                if '$ref' in variant:
                    # Extract the schema name from the reference
                    ref_name = variant['$ref'].split('/')[-1]
                    
                    # Get the discriminator value for this variant
                    ref_schema = schemas.get(ref_name, {})
                    properties = ref_schema.get('properties', {})
                    
                    # Try to determine the discriminator value
                    discriminator_value = None
                    if property_name in properties:
                        prop_def = properties[property_name]
                        if 'const' in prop_def:
                            discriminator_value = prop_def['const']
                        elif 'enum' in prop_def and len(prop_def['enum']) == 1:
                            discriminator_value = prop_def['enum'][0]
                    
                    # If we can't find the discriminator value, use a heuristic
                    if not discriminator_value:
                        # Try to extract from the schema name
                        # e.g., AssistantToolsCode -> code_interpreter
                        if ref_name.startswith(schema_name):
                            suffix = ref_name[len(schema_name):]
                            if suffix:
                                discriminator_value = suffix.lower()
                        else:
                            discriminator_value = ref_name.lower()
                    
                    variant_info.append({
                        'schema': ref_name,
                        'discriminator_value': discriminator_value
                    })
            
            if variant_info:
                empty_enums[schema_name] = {
                    'property_name': property_name,
                    'variants': variant_info
                }
    
    return empty_enums

def fix_case_sensitivity_in_enum(enum_file, enum_name, enum_info, content):
    """Fix case sensitivity issues in existing enum variants."""
    original_content = content
    fixed_any = False
    
    # Find all Box<models::TypeName> patterns and fix any with incorrect acronym casing
    pattern = r'Box<models::([A-Za-z]+)>'
    
    def fix_type_name(match):
        type_name = match.group(1)
        fixed_name = convert_to_rust_type_name(type_name)
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

def fix_empty_enum(models_dir, enum_name, enum_info):
    """Fix a single empty enum by adding proper variants."""
    # Convert to snake_case for the file name
    file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', enum_name).lower() + '.rs'
    enum_file = models_dir / file_name
    
    if not enum_file.exists():
        print(f"Warning: File {enum_file} not found for enum {enum_name}")
        return False
    
    with open(enum_file, 'r') as f:
        content = f.read()
    
    # Check if enum is empty
    is_empty = bool(re.search(rf'pub enum {enum_name} \{{\s*\}}', content))
    if not is_empty:
        # Even if not empty, fix case sensitivity issues in existing variants
        return fix_case_sensitivity_in_enum(enum_file, enum_name, enum_info, content)
    
    # Build the new enum with variants
    variants = []
    for variant in enum_info['variants']:
        schema_name = variant['schema']
        discriminator_value = variant['discriminator_value']
        
        # Convert schema name to proper Rust type name
        # Handle special cases like MCP -> Mcp
        rust_type_name = convert_to_rust_type_name(schema_name)
        
        # Generate a variant name (remove common prefix if it exists)
        variant_name = schema_name
        if schema_name.startswith(enum_name):
            variant_name = schema_name[len(enum_name):]
        if not variant_name:
            variant_name = schema_name
        
        # Apply the same conversion rules to the variant name
        variant_name = convert_to_rust_type_name(variant_name)
        
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
    
    print("Detecting empty tagged enums from OpenAPI spec...")
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