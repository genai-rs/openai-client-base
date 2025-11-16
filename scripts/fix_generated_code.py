#!/usr/bin/env python3
"""
Fix common issues in OpenAPI Generator output for Rust.
This script is run after code generation to make the code compile.
"""

import os
import re
from pathlib import Path

def fix_recursive_grammar_format(models_dir):
    """Fix recursive GrammarFormat types by using Box for indirection."""
    # Handle both grammar_format.rs and grammar_format_1.rs
    for filename in ["grammar_format.rs", "grammar_format_1.rs"]:
        grammar_file = models_dir / filename
        if not grammar_file.exists():
            continue

        with open(grammar_file, 'r') as f:
            content = f.read()

        original_content = content

        # Fix direct recursion: GrammarFormat containing GrammarFormat
        content = re.sub(
            r'pub grammar: models::GrammarFormat,',
            r'pub grammar: Box<models::GrammarFormat>,',
            content
        )

        # Fix GrammarFormat1 -> GrammarFormat reference
        content = re.sub(
            r'pub grammar: models::GrammarFormat1,',
            r'pub grammar: Box<models::GrammarFormat>,',
            content
        )

        # Fix constructor parameter types
        content = re.sub(
            r'pub fn new\(r#type: Type, grammar: models::GrammarFormat1\)',
            r'pub fn new(r#type: Type, grammar: models::GrammarFormat)',
            content
        )
        content = re.sub(
            r'pub fn new\(r#type: Type, grammar: models::GrammarFormat\) -> (GrammarFormat1?)\s*\{',
            r'pub fn new(r#type: Type, grammar: models::GrammarFormat) -> \1 {',
            content
        )

        # Fix constructor body to Box::new the grammar
        if 'pub grammar: Box<models::GrammarFormat>' in content:
            content = re.sub(
                r'(\s+)(r#type,\s+grammar),',
                r'\1r#type,\n\1grammar: Box::new(grammar),',
                content
            )
            content = re.sub(
                r'(\s+grammar:) grammar,',
                r'\1 Box::new(grammar),',
                content
            )

        if content != original_content:
            with open(grammar_file, 'w') as f:
                f.write(content)
            print(f"Fixed recursive type in {filename}")

def fix_invalid_enum_variants(models_dir):
    """Fix enum variant names with dots/hyphens and leading digits (e.g., Gpt4.1 -> Gpt4_1, 24h -> Variant24h)."""
    for file_path in models_dir.glob("*.rs"):
        with open(file_path, 'r') as f:
            content = f.read()

        original_content = content

        # Fix dots in enum variant names
        content = re.sub(r'\bGpt4\.1\b', 'Gpt4_1', content)
        content = re.sub(r'\bGpt4\.5\b', 'Gpt4_5', content)
        content = re.sub(r'\bGpt3\.5\b', 'Gpt3_5', content)

        # Fix hyphens in type names (e.g., models::ConversationParam-2 -> models::ConversationParam2)
        # Only match in type contexts (after models::, after Box<, in enum variants, field types)
        # This prevents changing unrelated hyphens like in comments or strings
        content = re.sub(r'(models::)([A-Z]\w+)-(\d+)', r'\1\2\3', content)
        content = re.sub(r'(Box<models::)([A-Z]\w+)-(\d+)', r'\1\2\3', content)

        # Fix enum variants that start with a digit (invalid Rust identifiers)
        numeric_variant_pattern = re.compile(r'(?m)^(\s*)(\d[\w]*)\s*,\s*$')
        matches = list(numeric_variant_pattern.finditer(content))
        if matches:
            for match in reversed(matches):
                indent, raw_name = match.groups()
                sanitized = f"Variant{re.sub(r'[^A-Za-z0-9_]', '_', raw_name)}"

                # Skip if we've already rewritten this variant
                existing_pattern = rf'^{indent}(?:#\[serde\(rename = "{re.escape(raw_name)}"\)\]\n{indent})?{re.escape(sanitized)}\s*,'
                if re.search(existing_pattern, content, re.MULTILINE):
                    continue

                replacement = f'{indent}#[serde(rename = "{raw_name}")]\n{indent}{sanitized},'
                content = content[:match.start()] + replacement + content[match.end():]

                # Update references to the old variant name within the file
                content = re.sub(
                    rf'(::|Self::){re.escape(raw_name)}\b',
                    rf'\1{sanitized}',
                    content,
                )

        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Fixed enum variants in {file_path.name}")

def add_display_impl_for_structs(models_dir):
    """Ensure all serializable structs have a Display impl that JSON-encodes the value.

    This keeps multipart form parameters and other to_string() usages working automatically
    for any new struct emitted by the generator without manual curation.
    """

    struct_pattern = re.compile(r'pub struct (\w+)')

    for file_path in models_dir.glob('*.rs'):
        content = file_path.read_text()

        if 'pub struct ' not in content:
            continue

        # Skip files that already implement Display for all structs
        structs = struct_pattern.findall(content)
        if not structs:
            continue

        added = []
        for struct_name in structs:
            # Only add Display if struct derives Serialize
            derive_regex = re.compile(rf'#\[derive\(([^)]*?)\)\]\s*pub struct {struct_name}\b', re.DOTALL)
            derive_match = derive_regex.search(content)
            if derive_match:
                derive_clause = derive_match.group(1)
                if 'Serialize' not in derive_clause:
                    continue
            else:
                continue

            if f"impl std::fmt::Display for {struct_name}" in content:
                continue

            impl_body = f'''

impl std::fmt::Display for {struct_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        match serde_json::to_string(self) {{
            Ok(s) => write!(f, "{{}}", s),
            Err(_) => Err(std::fmt::Error),
        }}
    }}
}}
'''
            added.append(impl_body)

        if added:
            file_path.write_text(content + ''.join(added))
            print(f"Added JSON Display impls in {file_path.name}")


def fix_manual_option_box_map(models_dir):
    """Replace verbose Option<Box<T>> patterns with map(Box::new)."""
    pattern = re.compile(
        r"if let Some\(x\) = (?P<var>[A-Za-z_][A-Za-z0-9_]*) \{\s*"
        r"Some\(Box::new\(x\)\)\s*\}\s*else \{\s*None\s*\}",
        re.DOTALL,
    )

    for file_path in models_dir.glob("*.rs"):
        content = file_path.read_text()
        new_content, count = pattern.subn(
            lambda m: f"{m.group('var')}.map(Box::new)",
            content,
        )
        if count > 0:
            file_path.write_text(new_content)
            print(f"Simplified Option::map pattern in {file_path.name} ({count} occurrences)")


def remove_default_from_empty_enums(models_dir):
    """Remove Default impl from empty enums and fix double closing braces."""
    for file_path in models_dir.glob("*.rs"):
        with open(file_path, 'r') as f:
            content = f.read()
        
        modified = False
        
        # Check if this has an empty enum
        if re.search(r'pub enum \w+ \{\s*\}', content):
            # Remove Default impl if present
            new_content = re.sub(
                r'impl Default for (\w+) \{[^}]*\}\s*\}\s*',
                '',
                content,
                flags=re.DOTALL
            )
            
            # Fix double closing braces
            new_content = re.sub(
                r'(pub enum \w+ \{\s*\})\s*\n\s*\}',
                r'\1',
                new_content
            )
            
            if new_content != content:
                with open(file_path, 'w') as f:
                    f.write(new_content)
                modified = True
                print(f"Fixed empty enum in {file_path.name}")

def remove_default_from_problematic_structs(models_dir):
    """Remove Default derive from structs that contain non-Default fields."""
    
    # List of types that no longer have Default
    non_default_types = set()
    
    # First pass: identify empty enums
    for file_path in models_dir.glob("*.rs"):
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Check for empty enum
        match = re.search(r'pub enum (\w+) \{\s*\}', content)
        if match:
            non_default_types.add(match.group(1))
    
    # Keep iterating until no more changes (to handle transitive dependencies)
    changes = True
    iterations = 0
    while changes and iterations < 10:  # Max 10 iterations to prevent infinite loop
        changes = False
        iterations += 1
        
        # Find structs that have fields with non-Default types
        for file_path in models_dir.glob("*.rs"):
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Extract struct name
            struct_match = re.search(r'pub struct (\w+)', content)
            if not struct_match:
                continue
            struct_name = struct_match.group(1)
            
            # Check if this struct has Default derive
            if not re.search(r'#\[derive\([^)]*Default[^)]*\)\]', content):
                # Already doesn't have Default, add to set
                if struct_name not in non_default_types:
                    non_default_types.add(struct_name)
                continue
            
            # Check if file contains any of the non-default types as fields
            should_remove_default = False
            for type_name in non_default_types:
                # Pattern to match field with this type (including models:: prefix)
                field_pattern = rf'pub \w+: (?:Option<)?Box<(?:models::)?{type_name}>'
                if re.search(field_pattern, content):
                    should_remove_default = True
                    break
            
            if should_remove_default:
                # Remove Default from derive
                new_content = re.sub(
                    r'#\[derive\(([^)]*?)Default,\s*([^)]*)\)\]',
                    r'#[derive(\1\2)]',
                    content
                )
                new_content = re.sub(
                    r'#\[derive\(([^)]*?),\s*Default([^)]*)\)\]',
                    r'#[derive(\1\2)]',
                    new_content
                )
                
                # Clean up double commas
                new_content = re.sub(r',\s*,', ',', new_content)
                new_content = re.sub(r'\(\s*,', '(', new_content)
                new_content = re.sub(r',\s*\)', ')', new_content)
                
                if new_content != content:
                    with open(file_path, 'w') as f:
                        f.write(new_content)
                    print(f"Removed Default derive from {file_path.name}")
                    non_default_types.add(struct_name)
                    changes = True

def main():
    project_root = Path(__file__).parent.parent
    models_dir = project_root / "src" / "models"
    
    if not models_dir.exists():
        print("No models directory found, skipping fixes")
        return
    
    print("Fixing generated Rust code...")
    
    # Apply fixes in order
    fix_invalid_enum_variants(models_dir)
    fix_recursive_grammar_format(models_dir)
    add_display_impl_for_structs(models_dir)
    fix_manual_option_box_map(models_dir)
    remove_default_from_empty_enums(models_dir)
    remove_default_from_problematic_structs(models_dir)
    
    print("\nCode fixes applied successfully!")

if __name__ == "__main__":
    main()
