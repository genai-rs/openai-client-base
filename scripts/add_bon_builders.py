#!/usr/bin/env python3
"""
Add bon::Builder derive to Create*Request structs in generated models.
Only applies to actual structs, not enums or sub-types.
"""

import os
import re
import sys
from pathlib import Path

def add_bon_builder_to_all_structs(root_dir):
    """Add bon::Builder to ALL structs in models directory."""
    models_dir = Path(root_dir) / "src" / "models"
    
    # Find all .rs files in models directory
    model_files = list(models_dir.glob("*.rs"))
    
    for file_path in model_files:
        # Skip mod.rs
        if file_path.name == "mod.rs":
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Skip if already has bon::Builder
        if 'bon::Builder' in content:
            continue
        
        # Find all pub struct definitions and add bon::Builder to their derive macros
        lines = content.split('\n')
        modified = False
        
        i = 0
        while i < len(lines):
            line = lines[i]
            
            # Look for derive macro followed by pub struct
            if line.startswith('#[derive(') and i + 1 < len(lines):
                next_line = lines[i + 1]
                
                # Check if next line is a pub struct (not enum)
                if re.match(r'^pub struct \w+ \{', next_line):
                    # Add bon::Builder to the derive
                    closing_paren = line.rfind(')')
                    if closing_paren != -1:
                        lines[i] = line[:closing_paren] + ', bon::Builder' + line[closing_paren:]
                        modified = True
                        struct_name = re.search(r'pub struct (\w+)', next_line).group(1)
                        print(f"Added bon::Builder to {struct_name} in {file_path.name}")
            
            i += 1
        
        if modified:
            with open(file_path, 'w') as f:
                f.write('\n'.join(lines))

def add_bon_builder_to_other_structs(root_dir):
    """Add bon::Builder to other commonly used request/response structs."""
    models_dir = Path(root_dir) / "src" / "models"
    
    # Additional structs that benefit from builders
    other_patterns = [
        "create_message_request.rs",
        "create_thread_request.rs",
        "create_run_request.rs",
        "submit_tool_outputs_run_request.rs",
        "modify_assistant_request.rs",
        "modify_thread_request.rs",
        "modify_message_request.rs",
        "modify_run_request.rs",
    ]
    
    for pattern in other_patterns:
        file_path = models_dir / pattern
        if not file_path.exists():
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Skip if already has bon::Builder
        if 'bon::Builder' in content:
            continue
        
        # Find the main struct in the file
        struct_match = re.search(r'^pub struct (\w+) \{', content, re.MULTILINE)
        if not struct_match:
            continue
        
        struct_name = struct_match.group(1)
        
        # Find the derive macro for this struct and add bon::Builder
        lines = content.split('\n')
        modified = False
        
        for i, line in enumerate(lines):
            if line.startswith('#[derive(') and i + 1 < len(lines):
                next_line = lines[i + 1]
                if next_line.startswith(f'pub struct {struct_name} {{'):
                    closing_paren = line.rfind(')')
                    if closing_paren != -1:
                        lines[i] = line[:closing_paren] + ', bon::Builder' + line[closing_paren:]
                        modified = True
                        print(f"Added bon::Builder to {struct_name} in {file_path.name}")
                        break
        
        if modified:
            with open(file_path, 'w') as f:
                f.write('\n'.join(lines))

if __name__ == "__main__":
    root_dir = sys.argv[1] if len(sys.argv) > 1 else "crates/openai-patched"
    
    print(f"Adding bon::Builder to ALL structs in {root_dir}")
    add_bon_builder_to_all_structs(root_dir)
    print("Done adding bon::Builder derives")