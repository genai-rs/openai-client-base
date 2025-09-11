#!/usr/bin/env python3
"""
Fix fields that should be Option<Box<T>> based on compilation errors.
These are fields that need to be nullable but aren't marked as such in the spec.
"""

import os
import re
from pathlib import Path

# These are the specific fields that need to be Option<Box<T>> based on compilation errors
FIELDS_TO_FIX = {
    'FineTuningJob': ['error'],
    'MessageObject': ['incomplete_details'],
    'RunObject': ['required_action', 'last_error', 'incomplete_details'],
    'RunStepObject': ['last_error'],
    'ThreadObject': ['tool_resources'],
    'VectorStoreFileObject': ['last_error'],
}

def snake_to_field(name):
    """Convert snake_case to field name for matching."""
    return name

def fix_boxed_field_to_option(file_path, field_names):
    """Fix Box<T> fields to be Option<Box<T>>."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    for field_name in field_names:
        # Convert to snake_case if needed
        snake_field = re.sub(r'(?<!^)(?=[A-Z])', '_', field_name).lower()
        
        # Pattern to match the field declaration
        # Match: pub field_name: Box<models::SomeType>,
        # Replace with: pub field_name: Option<Box<models::SomeType>>,
        field_pattern = rf'(\n\s+pub {snake_field}:\s+)(Box<[^>]+>)'
        
        def replacement(match):
            box_type = match.group(2)
            # Check if it's already wrapped in Option
            if 'Option<' in match.group(0):
                return match.group(0)
            return f"{match.group(1)}Option<{box_type}>"
        
        content = re.sub(field_pattern, replacement, content)
        
        # Also need to handle serde attributes
        # Find the line before the field and add skip_serializing_if if needed
        serde_pattern = rf'(#\[serde\(rename = "{snake_field}")\]\s*\n\s*pub {snake_field}:\s+Option<'
        if re.search(serde_pattern, content):
            # Add skip_serializing_if if not present
            serde_with_skip = rf'(#\[serde\(rename = "{snake_field}")'
            if f'rename = "{snake_field}"' in content and 'skip_serializing_if' not in content:
                content = re.sub(serde_with_skip, rf'\1, skip_serializing_if = "Option::is_none"', content)
    
    # Now fix the constructor to match
    # The constructor should already have Option parameters from our previous fix,
    # but the body needs to use the Option directly, not wrap it again
    
    # Find the new() function
    new_func_pattern = r'(pub fn new\([^)]+\) -> \w+ \{[\s\S]+?\n    \})'
    new_func_match = re.search(new_func_pattern, content)
    
    if new_func_match:
        new_func = new_func_match.group(1)
        modified_func = new_func
        
        for field_name in field_names:
            snake_field = re.sub(r'(?<!^)(?=[A-Z])', '_', field_name).lower()
            
            # In the constructor body, the field should just use the parameter directly
            # since it's already Option<models::Type>
            # Change from: field: field.map(Box::new)
            # To: field: field.map(Box::new) (keep as is if parameter is Option)
            # OR from: field: Some(Box::new(field))
            # To: field: field.map(Box::new)
            
            # Pattern for Some(Box::new(...))
            body_pattern = rf'(\n\s+{snake_field}:\s+)Some\(Box::new\({snake_field}\)\)'
            modified_func = re.sub(body_pattern, rf'\1{snake_field}.map(Box::new)', modified_func)
            
            # If the field is already using .map(Box::new), keep it
            # This handles the case where the parameter is Option<T> and needs Box wrapping
        
        content = content.replace(new_func, modified_func)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    
    return False

def main():
    if len(os.sys.argv) < 2:
        print("Usage: fix_boxed_nullable_fields.py <root_dir>")
        os.sys.exit(1)
    
    root_dir = Path(os.sys.argv[1])
    models_dir = root_dir / 'src' / 'models'
    
    if not models_dir.exists():
        print(f"Models directory not found: {models_dir}")
        os.sys.exit(1)
    
    print("Fixing boxed nullable fields...")
    
    fixed_count = 0
    for struct_name, fields in FIELDS_TO_FIX.items():
        # Convert struct name to file name
        file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
        file_path = models_dir / file_name
        
        if not file_path.exists():
            print(f"Warning: File {file_path} not found")
            continue
        
        print(f"Fixing {struct_name} fields: {fields}")
        if fix_boxed_field_to_option(file_path, fields):
            fixed_count += 1
            print(f"  Fixed {struct_name}")
    
    print(f"Fixed {fixed_count} structs")

if __name__ == '__main__':
    main()