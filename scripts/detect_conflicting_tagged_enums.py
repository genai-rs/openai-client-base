#!/usr/bin/env python3
"""
Detect enums with #[serde(tag = "type")] that wrap structs which also have a 'type' field.
This causes serialization conflicts.
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
    if re.search(r'pub\s+(r#)?type\s*:', content):
        return True

    return False


def analyze_enum_file(filepath, src_dir):
    """Analyze an enum file to see if it has conflicting tagged serialization."""
    with open(filepath, 'r') as f:
        content = f.read()

    # Check if it has #[serde(tag = "type")]
    if '#[serde(tag = "type")]' not in content:
        return None

    # Extract enum name
    enum_match = re.search(r'pub enum (\w+)', content)
    if not enum_match:
        return None

    enum_name = enum_match.group(1)

    # Find all variants that wrap types
    variant_pattern = r'(\w+)\(Box<models::(\w+)>\)'
    variants = re.findall(variant_pattern, content)

    if not variants:
        return None

    # Check if any wrapped type has a 'type' field
    conflicting_types = []
    for variant_name, wrapped_type in variants:
        if has_type_field(wrapped_type, src_dir):
            conflicting_types.append((variant_name, wrapped_type))

    if conflicting_types:
        return {
            'file': os.path.basename(filepath),
            'enum': enum_name,
            'conflicts': conflicting_types
        }

    return None


def main():
    if len(sys.argv) != 2:
        print("Usage: detect_conflicting_tagged_enums.py <project_root>")
        sys.exit(1)

    project_root = sys.argv[1]
    src_dir = os.path.join(project_root, "src", "models")

    if not os.path.isdir(src_dir):
        print(f"Error: {src_dir} is not a directory")
        sys.exit(1)

    print("Scanning for conflicting tagged enums...\n")

    conflicts = []
    for filename in os.listdir(src_dir):
        if filename.endswith('.rs'):
            filepath = os.path.join(src_dir, filename)
            result = analyze_enum_file(filepath, src_dir)
            if result:
                conflicts.append(result)

    if conflicts:
        print(f"Found {len(conflicts)} enum(s) with conflicts:\n")
        for conflict in conflicts:
            print(f"📁 {conflict['file']}")
            print(f"   Enum: {conflict['enum']}")
            print(f"   Conflicting variants:")
            for variant_name, wrapped_type in conflict['conflicts']:
                print(f"     - {variant_name} wraps {wrapped_type} (has 'type' field)")
            print()
    else:
        print("✓ No conflicts found")


if __name__ == "__main__":
    main()
