#!/usr/bin/env python3
"""
Fix structs that can't derive Default due to fields that don't implement Default.
Either make the field Option or remove Default derive.
"""

import os
import re
from pathlib import Path

# Structs that have fields that don't implement Default
# These need to have Default removed from their derive (seed list)
REMOVE_DEFAULT_FROM = [
    'FineTuneReinforcementMethod',
    'GraderMulti',
    'RealtimeBetaServerEventConversationItemInputAudioTranscriptionCompleted',
    'RealtimeServerEventConversationItemInputAudioTranscriptionCompleted',
    'RunObject',
    'ValidateGraderRequest',
    # Stream events containing RunObject
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
    # First, apply to the seed list
    for struct_name in REMOVE_DEFAULT_FROM:
        file_name = re.sub(r'(?<!^)(?=[A-Z])', '_', struct_name).lower() + '.rs'
        file_path = models_dir / file_name
        if file_path.exists() and remove_default_derive(file_path):
            fixed_count += 1
            print(f"  Removed Default from {struct_name}")

    # Then, auto-detect any struct that has a field with Box<models::RunObject> or ChatCompletion*MessageContent
    for file_path in models_dir.glob('*.rs'):
        content = file_path.read_text()
        if 'derive' in content and 'Default' in content:
            if ('Box<models::RunObject>' in content or re.search(r'Box<models::ChatCompletionRequest\w*MessageContent>', content)):
                if remove_default_derive(file_path):
                    fixed_count += 1
                    print(f"  Removed Default from {file_path.name} (non-Default boxed field)")

    print(f"Fixed Default derives in {fixed_count} files")

if __name__ == '__main__':
    main()
