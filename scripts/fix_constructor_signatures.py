#!/usr/bin/env python3
"""
Fix constructor signatures to match nullable fields that were converted to Option<Box<T>>.
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

def fix_constructor_for_nullable_fields(file_path, struct_name, nullable_fields):
    """Fix constructor signature and implementation for nullable fields."""
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    for field_name in nullable_fields:
        # Convert field name from camelCase to snake_case
        snake_field = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', field_name).lower()
        
        # In the new() function signature, change the parameter type to Option
        # From: field_name: models::SomeType,
        # To: field_name: Option<models::SomeType>,
        pattern = rf'(\n\s+{snake_field}:\s+)(models::[^,\)]+)'
        
        def replacement(match):
            param_type = match.group(2)
            # Check if it's already wrapped in Option
            if 'Option<' in param_type:
                return match.group(0)
            return f"{match.group(1)}Option<{param_type}>"
        
        content = re.sub(pattern, replacement, content)
        
        # In the constructor body, wrap the parameter with Option and Box if needed
        # Change from: field_name: Box::new(field_name),
        # To: field_name: field_name.map(Box::new),
        pattern = rf'(\n\s+{snake_field}:\s+)Some\(Box::new\({snake_field}\)\)'
        replacement = rf'\1{snake_field}.map(Box::new)'
        content = re.sub(pattern, replacement, content)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    
    return False

def main():
    if len(os.sys.argv) < 2:
        print("Usage: fix_constructor_signatures.py <root_dir> [spec_path]")
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
    
    # Focus on the structs that have constructor issues
    problematic_structs = [
        'RunObject',
        'MessageObject', 
        'ThreadObject',
        'VectorStoreFileObject',
        'FineTuningJob'
    ]
    
    fixed_count = 0
    for struct_name in problematic_structs:
        if struct_name not in nullable_fields:
            continue
            
        file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
        file_path = models_dir / file_name
        
        if not file_path.exists():
            print(f"Warning: File {file_path} not found")
            continue
        
        print(f"Fixing constructor for {struct_name}...")
        if fix_constructor_for_nullable_fields(file_path, struct_name, nullable_fields[struct_name]):
            fixed_count += 1
            print(f"  Fixed {struct_name}")
    
    print(f"Fixed {fixed_count} constructors")

if __name__ == '__main__':
    main()