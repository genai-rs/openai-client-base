#!/usr/bin/env python3
"""
Fix common issues in OpenAPI Generator output for Rust.
This script is run after code generation to make the code compile.
"""

import os
import re
from pathlib import Path

def fix_recursive_grammar_format(models_dir):
    """Fix the recursive GrammarFormat1 type by using GrammarFormat instead."""
    grammar_file = models_dir / "grammar_format_1.rs"
    if grammar_file.exists():
        with open(grammar_file, 'r') as f:
            content = f.read()
        
        # Fix the recursive field
        content = re.sub(
            r'pub grammar: models::GrammarFormat1,',
            r'pub grammar: Box<models::GrammarFormat>,',
            content
        )
        
        # Fix the constructor
        content = re.sub(
            r'pub fn new\(r#type: Type, grammar: models::GrammarFormat1\) -> GrammarFormat1 \{',
            r'pub fn new(r#type: Type, grammar: models::GrammarFormat) -> GrammarFormat1 {',
            content
        )
        content = re.sub(
            r'(\s+)grammar,',
            r'\1grammar: Box::new(grammar),',
            content
        )
        
        with open(grammar_file, 'w') as f:
            f.write(content)
        print(f"Fixed recursive type in grammar_format_1.rs")

def add_display_impl_for_structs(models_dir):
    """Add Display implementation for structs that need it for multipart forms."""
    structs_needing_display = [
        'TranscriptionChunkingStrategy',
        'FileExpirationAfter',
    ]
    
    for struct_name in structs_needing_display:
        # Convert to snake_case filename
        file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
        file_path = models_dir / file_name
        
        if file_path.exists():
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Check if Display is already implemented
            if 'impl std::fmt::Display' not in content:
                # Add Display implementation using serde_json
                display_impl = f'''

impl std::fmt::Display for {struct_name} {{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
        write!(f, "{{}}", serde_json::to_string(self).unwrap())
    }}
}}
'''
                content += display_impl
                
                with open(file_path, 'w') as f:
                    f.write(content)
                print(f"Added Display impl for {struct_name}")

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
    fix_recursive_grammar_format(models_dir)
    add_display_impl_for_structs(models_dir)
    remove_default_from_empty_enums(models_dir)
    remove_default_from_problematic_structs(models_dir)
    
    print("\nCode fixes applied successfully!")

if __name__ == "__main__":
    main()