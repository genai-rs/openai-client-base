#!/usr/bin/env python3
"""
Fix OpenAPI Generator's TODO comments for file upload parameters.

The generator leaves TODO comments for binary format parameters that are missing
the x-oaiTypeLabel: file annotation. This script finds those TODOs and generates
the proper file upload code using multipart_helper::add_file_to_form.
"""

import re
import sys
from pathlib import Path


def fix_file_upload_todo(file_path):
    """Find and fix TODO comments for file upload parameters"""
    with open(file_path, 'r') as f:
        content = f.read()

    original_content = content

    # Check if multipart_helper is imported
    needs_import = False

    # Pattern to match:
    # 1. Variable assignment: let p_form_XXX = XXX;
    # 2. TODO comment mentioning that parameter
    # The parameter should be PathBuf type (file parameter)

    # Find function signatures with PathBuf parameters
    func_pattern = r'pub async fn (\w+)\([^)]*?(\w+):\s+(?:Option<)?std::path::PathBuf(?:>)?[^)]*\)'
    functions = re.finditer(func_pattern, content)

    changes_made = False

    for func_match in functions:
        func_name = func_match.group(1)
        param_name = func_match.group(2)

        # Look for the TODO comment for this parameter
        todo_pattern = rf"(\s*)//\s*TODO:\s*support file upload for '({param_name})' parameter"
        todo_match = re.search(todo_pattern, content)

        if not todo_match:
            continue

        indent = todo_match.group(1)
        param_name_in_todo = todo_match.group(2)

        # The variable name will be p_form_{param_name}
        var_name = f"p_form_{param_name_in_todo}"

        # Check if this parameter is Option<PathBuf> or just PathBuf
        # by looking at the function signature
        func_sig_start = content.rfind('pub async fn', 0, todo_match.start())
        func_sig_end = content.find(')', func_sig_start)
        func_sig = content[func_sig_start:func_sig_end]

        is_optional = f'{param_name}: Option<std::path::PathBuf>' in func_sig

        if is_optional:
            # Generate code for optional file parameter
            replacement = (
                f'{indent}if let Some(file_path) = {var_name} {{\n'
                f'{indent}    multipart_form = multipart_helper::add_file_to_form(multipart_form, &file_path, "{param_name_in_todo}")?;\n'
                f'{indent}}}'
            )
        else:
            # Generate code for required file parameter
            replacement = (
                f'{indent}multipart_form = multipart_helper::add_file_to_form(multipart_form, &{var_name}, "{param_name_in_todo}")?;'
            )

        # Replace the TODO comment with the actual implementation
        content = content.replace(todo_match.group(0), replacement)
        changes_made = True
        needs_import = True

        print(f"  Fixed file upload TODO for parameter '{param_name_in_todo}' in {file_path.name}::{func_name}")

    # Add multipart_helper import if needed and not already present
    if needs_import:
        # Check specifically for multipart_helper in the imports section
        import_has_multipart = re.search(r'use super::\{[^}]*multipart_helper', content)
        if import_has_multipart:
            print(f"  multipart_helper already imported in {file_path.name}")
        else:
            # Find the use super:: line and add multipart_helper to it
            # The imports might be in any order (cargo fmt sorts them)
            import_match = re.search(r'use super::\{([^}]+)\};', content)
            if import_match:
                imports = import_match.group(1)
                # Add multipart_helper to the imports
                new_imports = imports + ', multipart_helper'
                old_line = import_match.group(0)
                new_line = f'use super::{{{new_imports}}};'
                content = content.replace(old_line, new_line)
                print(f"  Added multipart_helper import to {file_path.name}")
                changes_made = True
            else:
                print(f"  Warning: No use super:: import found in {file_path.name}")

    if changes_made:
        with open(file_path, 'w') as f:
            f.write(content)
        return True

    return False


def main():
    project_root = Path(__file__).parent.parent
    apis_dir = project_root / "src" / "apis"

    if not apis_dir.exists():
        print(f"Error: APIs directory not found at {apis_dir}")
        sys.exit(1)

    print("Fixing file upload TODO comments...")

    total_fixed = 0
    for api_file in apis_dir.glob("*_api.rs"):
        if fix_file_upload_todo(api_file):
            total_fixed += 1

    if total_fixed > 0:
        print(f"✓ Fixed file upload TODOs in {total_fixed} file(s)")
    else:
        print("✓ No file upload TODOs found")


if __name__ == "__main__":
    main()
