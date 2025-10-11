#!/usr/bin/env python3
"""
Fix enums that incorrectly use internally-tagged serialization.

OpenAPI Generator generates enums with #[serde(tag = "type")] that wrap structs
which already have their own 'type' field, causing serialization conflicts.

This script automatically detects and fixes all such enums by converting them
to use #[serde(untagged)] instead.
"""

import os
import re
import sys


def has_type_field(struct_name, src_dir):
    """Check if a struct has a 'type' field by reading its source file."""
    # Convert struct name to snake_case filename
    filename = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
    filepath = os.path.join(src_dir, filename)

    if not os.path.exists(filepath):
        return False

    with open(filepath, 'r') as f:
        content = f.read()

    # Look for pub r#type or pub type field
    return bool(re.search(r'pub\s+(r#)?type\s*:', content))


def should_fix_enum(filepath, src_dir):
    """
    Determine if an enum file needs fixing by checking if:
    1. It uses #[serde(tag = "type")]
    2. Any of its wrapped types have a 'type' field
    """
    with open(filepath, 'r') as f:
        content = f.read()

    # Check if it has #[serde(tag = "type")]
    if '#[serde(tag = "type")]' not in content:
        return False, []

    # Find all variants that wrap types
    variant_pattern = r'(\w+)\(Box<models::(\w+)>\)'
    variants = re.findall(variant_pattern, content)

    if not variants:
        return False, []

    # Check if any wrapped type has a 'type' field
    conflicting_types = []
    for variant_name, wrapped_type in variants:
        if has_type_field(wrapped_type, src_dir):
            conflicting_types.append((variant_name, wrapped_type))

    return len(conflicting_types) > 0, conflicting_types


def fix_enum_tagging(file_path):
    """
    Replace #[serde(tag = "type")] with #[serde(untagged)] and remove variant renames.
    """
    with open(file_path, 'r') as f:
        content = f.read()

    original_content = content

    # Replace #[serde(tag = "type")] with #[serde(untagged)]
    content = re.sub(
        r'#\[serde\(tag = "type"\)\]',
        '#[serde(untagged)]',
        content
    )

    # Remove #[serde(rename = "...")] attributes from enum variants
    # This is needed because untagged enums don't use discriminator-based matching
    content = re.sub(
        r'    #\[serde\(rename = "[^"]+"\)\]\n',
        '',
        content
    )

    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    return False


def main():
    if len(sys.argv) != 2:
        print("Usage: fix_tool_enum_tagging.py <project_root>")
        sys.exit(1)

    project_root = sys.argv[1]
    src_dir = os.path.join(project_root, "src", "models")

    if not os.path.isdir(src_dir):
        print(f"Error: {src_dir} is not a directory")
        sys.exit(1)

    print("Scanning for enums with conflicting tagged serialization...\n")

    files_to_fix = []
    for filename in os.listdir(src_dir):
        if filename.endswith('.rs'):
            filepath = os.path.join(src_dir, filename)
            needs_fix, conflicts = should_fix_enum(filepath, src_dir)
            if needs_fix:
                files_to_fix.append((filename, conflicts))

    if not files_to_fix:
        print("✓ No conflicting enums found")
        return

    print(f"Found {len(files_to_fix)} file(s) to fix:\n")

    fixed_count = 0
    for filename, conflicts in files_to_fix:
        file_path = os.path.join(src_dir, filename)
        print(f"  {filename}")
        for variant_name, wrapped_type in conflicts:
            print(f"    - {variant_name} → {wrapped_type}")

        if fix_enum_tagging(file_path):
            fixed_count += 1

    print(f"\n✓ Fixed {fixed_count} file(s)")


if __name__ == "__main__":
    main()
