#!/usr/bin/env python3
"""
Fix structs that can't derive Default due to fields that don't implement Default.
Either make the field Option or remove Default derive.
"""

import os
import re
from pathlib import Path

# Structs that have fields that don't implement Default
# These need to have Default removed from their derive
REMOVE_DEFAULT_FROM = [
    'FineTuneReinforcementMethod',  # has grader: Box<FineTuneReinforcementMethodGrader>
    'GraderMulti',  # has graders: Box<GraderMultiGraders>
    'RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompleted',  # has usage
    'RealtimeServerEventConversationItemInputAudioTranscriptionCompleted',  # has usage
    'RunObject',  # has tool_choice: Box<AssistantsApiToolChoiceOption>
    'ValidateGraderRequest',  # has grader field
    # Stream events that contain RunObject
    'RunStreamEventAnyOf',
    'RunStreamEventAnyOf1',
    'RunStreamEventAnyOf2',
    'RunStreamEventAnyOf3',
    'RunStreamEventAnyOf4',
    'RunStreamEventAnyOf5',
    'RunStreamEventAnyOf6',
    'RunStreamEventAnyOf7',
    'RunStreamEventAnyOf8',
    'RunStreamEventAnyOf9',
]

def remove_default_derive(file_path):
    """Remove Default from the derive macro."""
    with open(file_path, 'r') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match derive with Default
    # Match: #[derive(..., Default, ...)]
    # Remove Default from the list
    derive_pattern = r'(#\[derive\([^)]*)\bDefault,?\s*([^)]*\)\])'
    
    def replacement(match):
        before = match.group(1)
        after = match.group(2)
        # Clean up extra commas
        before = re.sub(r',\s*,', ',', before)
        after = re.sub(r'^,\s*', '', after)
        return f"{before}{after}"
    
    content = re.sub(derive_pattern, replacement, content)
    
    # Also remove standalone Default if it's the only trait
    content = re.sub(r'#\[derive\(Default\)\]', '#[derive()]', content)
    
    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        return True
    
    return False

def main():
    if len(os.sys.argv) < 2:
        print("Usage: fix_default_issues.py <root_dir>")
        os.sys.exit(1)
    
    root_dir = Path(os.sys.argv[1])
    models_dir = root_dir / 'src' / 'models'
    
    if not models_dir.exists():
        print(f"Models directory not found: {models_dir}")
        os.sys.exit(1)
    
    print("Fixing Default derive issues...")
    
    fixed_count = 0
    for struct_name in REMOVE_DEFAULT_FROM:
        # Convert struct name to file name
        file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
        file_path = models_dir / file_name
        
        if not file_path.exists():
            print(f"Warning: File {file_path} not found")
            continue
        
        print(f"Removing Default from {struct_name}...")
        if remove_default_derive(file_path):
            fixed_count += 1
            print(f"  Fixed {struct_name}")
    
    print(f"Fixed {fixed_count} structs")

if __name__ == '__main__':
    main()