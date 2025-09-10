#!/usr/bin/env python3
"""
Add clippy allow attributes to generated code to suppress warnings.
Since this is auto-generated code, we don't want clippy failures in CI.
"""

import os
from pathlib import Path

def add_clippy_allows_to_lib(lib_file):
    """Add clippy allow attributes to lib.rs"""
    with open(lib_file, 'r') as f:
        content = f.read()
    
    # Remove old simple allows if they exist
    content = content.replace('#![allow(unused_imports)]', '')
    content = content.replace('#![allow(clippy::too_many_arguments)]', '')
    
    # Check if already has comprehensive allows
    if 'unreachable_patterns,' in content:
        print(f"  {lib_file} already has comprehensive clippy allows, skipping")
        return
    
    # Add comprehensive allows at the top of the file
    allows = """#![allow(
    unused_imports,
    unreachable_patterns,
    clippy::too_many_arguments,
    clippy::needless_return,
    clippy::unnecessary_operation,
    clippy::into_iter_on_ref,
    clippy::empty_docs,
    clippy::new_without_default,
    clippy::large_enum_variant,
    clippy::derive_partial_eq_without_eq,
    clippy::enum_variant_names,
    clippy::redundant_field_names,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]

"""
    
    # Find where to insert (after existing #![...] attributes if any)
    lines = content.split('\n')
    insert_pos = 0
    
    for i, line in enumerate(lines):
        if line.startswith('#!['):
            insert_pos = i + 1
        elif line and not line.startswith('#'):
            # First non-attribute, non-comment line
            break
    
    lines.insert(insert_pos, allows.rstrip())
    
    with open(lib_file, 'w') as f:
        f.write('\n'.join(lines))
    
    print(f"  Added clippy allows to {lib_file}")

def add_clippy_allows_to_models(models_dir):
    """Add clippy allow attributes to model files with unreachable patterns"""
    problem_files = [
        'item.rs',
        'realtime_conversation_item.rs',
    ]
    
    for filename in problem_files:
        file_path = models_dir / filename
        if not file_path.exists():
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check if already has the allow
        if '#[allow(unreachable_patterns)]' in content:
            print(f"  {filename} already has unreachable_patterns allow, skipping")
            continue
        
        # Add allow before the enum definition
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.startswith('pub enum'):
                # Insert allow attribute before the enum
                lines.insert(i, '#[allow(unreachable_patterns)]')
                break
        
        with open(file_path, 'w') as f:
            f.write('\n'.join(lines))
        
        print(f"  Added unreachable_patterns allow to {filename}")

def add_clippy_allows_to_apis(apis_dir):
    """Add clippy allow attributes to API files"""
    for file_path in apis_dir.glob('*.rs'):
        if file_path.name == 'mod.rs':
            continue  # Skip mod.rs for now
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check if already has allows
        if '#![allow(' in content or '#[allow(' in content:
            continue
        
        # Add allows at the top of the file after the header comment
        lines = content.split('\n')
        
        # Find end of header comment
        insert_pos = 0
        in_header = True
        for i, line in enumerate(lines):
            if in_header and line.strip() == '*/':
                insert_pos = i + 1
                in_header = False
                break
        
        if insert_pos > 0:
            lines.insert(insert_pos, '')
            lines.insert(insert_pos + 1, '#![allow(clippy::needless_return, clippy::into_iter_on_ref)]')
            
            with open(file_path, 'w') as f:
                f.write('\n'.join(lines))
            
            print(f"  Added clippy allows to {file_path.name}")

def main():
    project_root = Path(__file__).parent.parent
    
    print("Adding clippy allow attributes to generated code...")
    
    # Add to lib.rs
    lib_file = project_root / "src" / "lib.rs"
    if lib_file.exists():
        add_clippy_allows_to_lib(lib_file)
    
    # Add to specific model files with unreachable patterns
    models_dir = project_root / "src" / "models"
    if models_dir.exists():
        add_clippy_allows_to_models(models_dir)
    
    # Add to API files
    apis_dir = project_root / "src" / "apis"
    if apis_dir.exists():
        add_clippy_allows_to_apis(apis_dir)
    
    print("Done adding clippy allows!")

if __name__ == "__main__":
    main()