#!/usr/bin/env python3
"""
Fix untagged anyOf unions that OpenAPI Generator incorrectly generates as empty structs
or flattened structs. This converts them to proper Rust enums using serde's untagged feature.
"""

import os
import sys
import re

# Map of types that should be untagged enums with their variants
UNTAGGED_UNIONS = {
    'ChatCompletionRequestSystemMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
    ],
    'ChatCompletionRequestUserMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestUserMessageContentPart>'),
    ],
    'ChatCompletionRequestAssistantMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestAssistantMessageContentPart>'),
    ],
    'ChatCompletionRequestToolMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
    ],
    'ChatCompletionRequestDeveloperMessageContent': [
        ('Text', 'String'),
        ('Array', 'Vec<models::ChatCompletionRequestMessageContentPartText>'),
    ],
    'ChatCompletionRequestFunctionMessageContent': [
        ('Text', 'String'),
        ('Null', '()'),  # null type for function messages
    ],
    'ChatCompletionToolChoiceOption': [
        ('Auto', 'ChatCompletionToolChoiceOptionAuto'),
        ('Named', 'models::ChatCompletionNamedToolChoice'),
    ],
    'CreateEmbeddingRequestInput': [
        ('Text', 'String'),
        ('ArrayOfStrings', 'Vec<String>'),
        ('ArrayOfIntegers', 'Vec<i32>'),
        ('ArrayOfIntegerArrays', 'Vec<Vec<i32>>'),
    ],
    'AssistantsApiToolChoiceOption': [
        ('Auto', 'AssistantsApiToolChoiceOptionAuto'),
        ('Named', 'models::AssistantsNamedToolChoice'),
    ],
    'AssistantsApiResponseFormatOption': [
        ('Auto', 'AssistantsApiResponseFormatOptionAuto'),
        ('Text', 'AssistantsApiResponseFormatOptionText'),
        ('JsonObject', 'models::ResponseFormatJsonObject'),
        ('JsonSchema', 'models::ResponseFormatJsonSchema'),
    ],
}

# Simple string literals that can be enum variants
SIMPLE_STRING_ENUMS = {
    'ChatCompletionToolChoiceOptionAuto': ['none', 'auto', 'required'],
    'AssistantsApiToolChoiceOptionAuto': ['none', 'auto', 'required'],
    'AssistantsApiResponseFormatOptionAuto': ['auto'],
    'AssistantsApiResponseFormatOptionText': ['text'],
}

def create_untagged_enum(name, variants):
    """Generate an untagged enum for anyOf unions"""
    lines = [
        f"/// {name} - Untagged union type",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(untagged)]",
        f"pub enum {name} {{",
    ]
    
    for variant_name, variant_type in variants:
        if variant_type == '()':
            lines.append(f"    {variant_name},")
        else:
            lines.append(f"    {variant_name}({variant_type}),")
    
    lines.append("}")
    lines.append("")
    
    # Add Default implementation
    lines.extend([
        f"impl Default for {name} {{",
        f"    fn default() -> Self {{",
    ])
    if variants[0][1] == 'String':
        lines.append(f"        Self::Text(String::new())")
    elif variants[0][1] == '()':
        lines.append(f"        Self::{variants[0][0]}")
    else:
        lines.append(f"        Self::{variants[0][0]}(Default::default())")
    lines.extend([
        "    }",
        "}",
        "",
    ])
    
    # Add impl with new() if it's a simple type
    if len(variants) == 2 and variants[0][1] == 'String':
        lines.extend([
            f"impl {name} {{",
            f"    pub fn new_text(text: String) -> Self {{",
            f"        Self::Text(text)",
            f"    }}",
        ])
        if 'Array' in variants[1][0]:
            part_type = variants[1][1].replace('Vec<', '').replace('>', '')
            lines.extend([
                f"    pub fn new_array(array: Vec<{part_type}>) -> Self {{",
                f"        Self::Array(array)",
                f"    }}",
            ])
        lines.extend([
            "}",
            "",
            f"impl From<String> for {name} {{",
            f"    fn from(s: String) -> Self {{",
            f"        Self::Text(s)",
            f"    }}",
            f"}}",
            "",
            f"impl From<&str> for {name} {{",
            f"    fn from(s: &str) -> Self {{",
            f"        Self::Text(s.to_string())",
            f"    }}",
            f"}}",
        ])
    
    lines.append("")
    return '\n'.join(lines)

def create_simple_string_enum(name, values):
    """Generate a simple string enum"""
    lines = [
        f"/// {name} - String enum type",
        "#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]",
        "#[serde(rename_all = \"lowercase\")]",
        f"pub enum {name} {{",
    ]
    
    for value in values:
        variant = value.capitalize()
        if value == 'required':
            variant = 'Required'
        elif value == 'json_object':
            variant = 'JsonObject'
        lines.append(f"    {variant},")
    
    lines.extend([
        "}",
        "",
        f"impl Default for {name} {{",
        f"    fn default() -> Self {{",
        f"        Self::{values[0].capitalize()}",
        f"    }}",
        f"}}",
        "",
    ])
    
    return '\n'.join(lines)

def fix_file(filepath, root_dir):
    """Fix a single model file"""
    filename = os.path.basename(filepath).replace('.rs', '')
    
    # Convert filename to PascalCase
    type_name = ''.join(word.capitalize() for word in filename.split('_'))
    
    content = None
    
    # Check if this is an untagged union type
    if type_name in UNTAGGED_UNIONS:
        print(f"  Fixing untagged union: {type_name}")
        content = "use crate::models;\nuse serde::{Deserialize, Serialize};\n\n"
        content += create_untagged_enum(type_name, UNTAGGED_UNIONS[type_name])
        
        # Also create simple string enums if needed
        for variant_name, variant_type in UNTAGGED_UNIONS[type_name]:
            if variant_type in SIMPLE_STRING_ENUMS:
                content += create_simple_string_enum(variant_type, SIMPLE_STRING_ENUMS[variant_type])
    
    # Check if this is a simple string enum that needs to be created
    elif type_name in SIMPLE_STRING_ENUMS:
        print(f"  Creating simple string enum: {type_name}")
        content = "use serde::{Deserialize, Serialize};\n\n"
        content += create_simple_string_enum(type_name, SIMPLE_STRING_ENUMS[type_name])
    
    if content:
        with open(filepath, 'w') as f:
            f.write(content)
        return True
    
    return False

def main():
    if len(sys.argv) != 2:
        print("Usage: fix_untagged_unions.py <root_dir>")
        sys.exit(1)
    
    root_dir = sys.argv[1]
    models_dir = os.path.join(root_dir, 'src', 'models')
    
    if not os.path.exists(models_dir):
        print(f"Models directory not found: {models_dir}")
        sys.exit(1)
    
    print("Fixing untagged unions...")
    fixed_count = 0
    
    # First, create files for simple string enums that might not exist
    for enum_name in SIMPLE_STRING_ENUMS:
        filename = re.sub(r'([A-Z])', r'_\1', enum_name).lower().lstrip('_') + '.rs'
        filepath = os.path.join(models_dir, filename)
        if not os.path.exists(filepath):
            print(f"  Creating new file: {filename}")
            with open(filepath, 'w') as f:
                f.write("use serde::{Deserialize, Serialize};\n\n")
                f.write(create_simple_string_enum(enum_name, SIMPLE_STRING_ENUMS[enum_name]))
            fixed_count += 1
    
    # Now fix the untagged union types
    for type_name in UNTAGGED_UNIONS:
        filename = re.sub(r'([A-Z])', r'_\1', type_name).lower().lstrip('_') + '.rs'
        filepath = os.path.join(models_dir, filename)
        
        if os.path.exists(filepath):
            if fix_file(filepath, root_dir):
                fixed_count += 1
        else:
            print(f"  Warning: File not found for {type_name}: {filepath}")
    
    # Update mod.rs to include any new modules
    mod_rs_path = os.path.join(models_dir, 'mod.rs')
    if os.path.exists(mod_rs_path):
        with open(mod_rs_path, 'r') as f:
            content = f.read()
        
        for enum_name in SIMPLE_STRING_ENUMS:
            module_name = re.sub(r'([A-Z])', r'_\1', enum_name).lower().lstrip('_')
            pub_line = f"pub mod {module_name};"
            use_line = f"pub use self::{module_name}::{enum_name};"
            
            if pub_line not in content:
                # Find the right place to insert (alphabetically)
                lines = content.split('\n')
                inserted = False
                for i, line in enumerate(lines):
                    if line.startswith('pub mod ') and line > pub_line:
                        lines.insert(i, pub_line)
                        lines.insert(i + 1, use_line)
                        inserted = True
                        break
                if not inserted:
                    # Add at the end of mod declarations
                    for i, line in enumerate(lines):
                        if not line.startswith('pub mod ') and i > 0:
                            lines.insert(i - 1, pub_line)
                            lines.insert(i, use_line)
                            break
                
                with open(mod_rs_path, 'w') as f:
                    f.write('\n'.join(lines))
                print(f"  Added module {module_name} to mod.rs")
    
    print(f"Fixed {fixed_count} untagged union types")

if __name__ == '__main__':
    main()