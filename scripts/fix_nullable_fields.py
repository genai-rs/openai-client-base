#!/usr/bin/env python3
"""
Fix nullable fields in generated structs to be Option<Box<T>> instead of Box<T>.
This ensures structs can derive Default properly.
"""

import os
import re
from pathlib import Path
from utils import load_spec

def find_nullable_fields(spec):
    """Find all nullable fields in the OpenAPI spec."""
    nullable_fields = {}
    schemas = spec.get('components', {}).get('schemas', {})
    
    for schema_name, schema_def in schemas.items():
        if not isinstance(schema_def, dict):
            continue
            
        properties = schema_def.get('properties', {})
        for prop_name, prop_def in properties.items():
            if isinstance(prop_def, dict) and prop_def.get('nullable') == True:
                if schema_name not in nullable_fields:
                    nullable_fields[schema_name] = []
                nullable_fields[schema_name].append(prop_name)
    
    return nullable_fields

def fix_nullable_field(file_path, field_name):
    """Fix a single nullable field in a Rust file."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match the field declaration
    # Match: pub field_name: Box<SomeType>,
    # Replace with: pub field_name: Option<Box<SomeType>>,
    field_pattern = rf'(pub {field_name}:\s+)(Box<[^>]+>)'
    replacement = r'\1Option<\2>'
    content = re.sub(field_pattern, replacement, content)
    
    # Also fix it with serde rename
    # Match: #[serde(rename = "field_name")]
    #        pub field_name: Box<SomeType>,
    serde_pattern = rf'(#\[serde\(rename = "{field_name}".*?\]\s*\n\s*pub {field_name}:\s+)(Box<[^>]+>)'
    content = re.sub(serde_pattern, r'\1Option<\2>', content)
    
    # Add skip_serializing_if for Option fields
    if content != original_content:
        # Find the serde attribute and add skip_serializing_if
        serde_pattern = rf'(#\[serde\(rename = "{field_name}")'
        replacement = rf'\1, skip_serializing_if = "Option::is_none"'
        # Only add if not already present
        if 'skip_serializing_if' not in content:
            content = re.sub(serde_pattern, replacement, content)
    
    return content != original_content, content

def fix_struct_nullable_fields(models_dir, struct_name, nullable_fields):
    """Fix nullable fields in a struct."""
    # Convert to snake_case for file name
    file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
    file_path = models_dir / file_name
    
    if not file_path.exists():
        print(f"Warning: File {file_path} not found")
        return False
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    changes_made = False
    
    for field_name in nullable_fields:
        # Convert field name from camelCase to snake_case
        snake_field = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', field_name).lower()
        
        changed, content = fix_nullable_field(file_path, snake_field)
        if changed:
            changes_made = True
            print(f"  Fixed nullable field: {snake_field}")
    
    if changes_made:
        # Re-add Default derive if it was removed
        if 'Default' not in content and '#[derive(' in content:
            # Add Default back to the derive macro
            content = re.sub(
                r'(#\[derive\([^)]*)(PartialEq)',
                r'\1Default, \2',
                content
            )
            print(f"  Re-added Default derive")
        
        # Fix new() function calls for nullable fields
        for field_name in nullable_fields:
            snake_field = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', field_name).lower()
            # In new() function, wrap Box::new() calls with Some()
            pattern = rf'({snake_field}:\s+)(Box::new\([^)]+\))'
            replacement = r'\1Some(\2)'
            content = re.sub(pattern, replacement, content)
        
        with open(file_path, 'w') as f:
            f.write(content)
        
        return True
    
    return False

def main():
    if len(os.sys.argv) < 2:
        print("Usage: fix_nullable_fields.py <root_dir> [spec_path]")
        os.sys.exit(1)
    
    root_dir = Path(os.sys.argv[1])
    spec_path = Path(os.sys.argv[2]) if len(os.sys.argv) > 2 else root_dir / 'stainless.yaml'
    models_dir = root_dir / 'src' / 'models'
    
    if not models_dir.exists():
        print(f"Models directory not found: {models_dir}")
        os.sys.exit(1)
    
    if not spec_path.exists():
        print(f"OpenAPI spec not found: {spec_path}")
        os.sys.exit(1)
    
    print(f"Loading OpenAPI spec from: {spec_path}")
    spec = load_spec(str(spec_path))
    
    print("Finding nullable fields in spec...")
    nullable_fields = find_nullable_fields(spec)
    
    print(f"Found {len(nullable_fields)} schemas with nullable fields")
    
    fixed_count = 0
    for struct_name, fields in nullable_fields.items():
        print(f"Checking {struct_name} with {len(fields)} nullable fields...")
        if fix_struct_nullable_fields(models_dir, struct_name, fields):
            fixed_count += 1
    
    print(f"Fixed {fixed_count} structs with nullable fields")

if __name__ == '__main__':
    main()