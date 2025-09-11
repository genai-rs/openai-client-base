#!/usr/bin/env python3
"""
Remove Default derives from structs that contain fields without Default implementations.
"""

import os
import re
from pathlib import Path

def remove_default_from_problematic_structs(models_dir):
    """Remove Default derive from structs that are known to have fields without Default."""
    
    # List of structs that have fields without Default implementation
    # This includes both root problematic structs and those that contain them
    problematic_structs = [
        # Root problematic structs
        'FineTuneReinforcementMethod',
        'GraderMulti',
        'RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompleted',
        'RealtimeServerEventConversationItemInputAudioTranscriptionCompleted',
        'RunObject',
        'ValidateGraderRequest',
        # Structs that contain RunObject
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
    
    for struct_name in problematic_structs:
        # Convert to snake_case for file name
        file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
        file_path = models_dir / file_name
        
        if not file_path.exists():
            print(f"Warning: File {file_path} not found")
            continue
            
        with open(file_path, 'r') as f:
            content = f.read()
        
        # Remove Default from the derive macro
        # Match patterns like #[derive(..., Default, ...)] or #[derive(Default)]
        original_content = content
        
        # Pattern to match derive with Default
        pattern = r'(#\[derive\([^)]*?)(\s*,\s*Default|Default\s*,\s*)([^)]*?\)\])'
        content = re.sub(pattern, r'\1\3', content)
        
        # Also handle case where Default is the only derive
        content = re.sub(r'#\[derive\(Default\)\]', '', content)
        
        if content != original_content:
            with open(file_path, 'w') as f:
                f.write(content)
            print(f"Removed Default derive from {struct_name}")

def main():
    if len(os.sys.argv) < 2:
        print("Usage: fix_default_derives.py <root_dir>")
        os.sys.exit(1)
    
    root_dir = Path(os.sys.argv[1])
    models_dir = root_dir / 'src' / 'models'
    
    if not models_dir.exists():
        print(f"Models directory not found: {models_dir}")
        os.sys.exit(1)
    
    print("Removing Default derives from problematic structs...")
    remove_default_from_problematic_structs(models_dir)
    print("Done!")

if __name__ == '__main__':
    main()