#!/usr/bin/env python3
"""
Fix empty enums in OpenAPI Generator output by adding proper variants.
This script specifically handles tagged enums that the generator leaves empty.
"""

import os
import re
from pathlib import Path

def fix_assistant_tool_enum(models_dir):
    """Add proper variants to the AssistantTool enum."""
    assistant_tool_file = models_dir / "assistant_tool.rs"
    if assistant_tool_file.exists():
        # Read the current content
        with open(assistant_tool_file, 'r') as f:
            content = f.read()
        
        # Check if enum is empty
        if re.search(r'pub enum AssistantTool \{\s*\}', content):
            # Replace empty enum with proper variants
            new_enum = '''#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum AssistantTool {
    #[serde(rename = "code_interpreter")]
    AssistantToolsCode(Box<models::AssistantToolsCode>),
    #[serde(rename = "file_search")]
    AssistantToolsFileSearch(Box<models::AssistantToolsFileSearch>),
    #[serde(rename = "function")]
    AssistantToolsFunction(Box<models::AssistantToolsFunction>),
}'''
            
            content = re.sub(
                r'#\[derive\([^)]*\)\]\s*#\[serde\(tag = "type"\)\]\s*pub enum AssistantTool \{\s*\}',
                new_enum,
                content
            )
            
            with open(assistant_tool_file, 'w') as f:
                f.write(content)
            print(f"Fixed AssistantTool enum with proper variants")

def fix_chat_completion_message_content_part_enum(models_dir):
    """Add proper variants to ChatCompletionRequestUserMessageContentPart enum."""
    content_part_file = models_dir / "chat_completion_request_user_message_content_part.rs"
    if content_part_file.exists():
        with open(content_part_file, 'r') as f:
            content = f.read()
        
        # Check if enum is empty
        if re.search(r'pub enum ChatCompletionRequestUserMessageContentPart \{\s*\}', content):
            # Replace empty enum with proper variants
            new_enum = '''#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ChatCompletionRequestUserMessageContentPart {
    #[serde(rename = "text")]
    Text(Box<models::ChatCompletionRequestMessageContentPartText>),
    #[serde(rename = "image_url")]
    ImageUrl(Box<models::ChatCompletionRequestMessageContentPartImage>),
    #[serde(rename = "input_audio")]
    InputAudio(Box<models::ChatCompletionRequestMessageContentPartAudio>),
}'''
            
            content = re.sub(
                r'#\[derive\([^)]*\)\]\s*#\[serde\(tag = "type"\)\]\s*pub enum ChatCompletionRequestUserMessageContentPart \{\s*\}',
                new_enum,
                content
            )
            
            with open(content_part_file, 'w') as f:
                f.write(content)
            print(f"Fixed ChatCompletionRequestUserMessageContentPart enum with proper variants")

def fix_message_delta_content_inner_enum(models_dir):
    """Add proper variants to MessageDeltaObjectDeltaContentInner enum if it exists."""
    content_inner_file = models_dir / "message_delta_object_delta_content_inner.rs"
    if content_inner_file.exists():
        with open(content_inner_file, 'r') as f:
            content = f.read()
        
        # Check if enum is empty
        if re.search(r'pub enum MessageDeltaObjectDeltaContentInner \{\s*\}', content):
            # Replace empty enum with proper variants
            new_enum = '''#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageDeltaObjectDeltaContentInner {
    #[serde(rename = "text")]
    MessageDeltaContentTextObject(Box<models::MessageDeltaContentTextObject>),
    #[serde(rename = "image_file")]
    MessageDeltaContentImageFileObject(Box<models::MessageDeltaContentImageFileObject>),
    #[serde(rename = "image_url")]
    MessageDeltaContentImageUrlObject(Box<models::MessageDeltaContentImageUrlObject>),
}'''
            
            content = re.sub(
                r'#\[derive\([^)]*\)\]\s*#\[serde\(tag = "type"\)\]\s*pub enum MessageDeltaObjectDeltaContentInner \{\s*\}',
                new_enum,
                content
            )
            
            with open(content_inner_file, 'w') as f:
                f.write(content)
            print(f"Fixed MessageDeltaObjectDeltaContentInner enum with proper variants")

def fix_message_content_delta_enum(models_dir):
    """Add proper variants to MessageContentDelta enum."""
    content_delta_file = models_dir / "message_content_delta.rs"
    if content_delta_file.exists():
        with open(content_delta_file, 'r') as f:
            content = f.read()
        
        # Check if enum is empty
        if re.search(r'pub enum MessageContentDelta \{\s*\}', content):
            # Replace empty enum with proper variants
            new_enum = '''#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageContentDelta {
    #[serde(rename = "text")]
    MessageDeltaContentTextObject(Box<models::MessageDeltaContentTextObject>),
    #[serde(rename = "image_file")]
    MessageDeltaContentImageFileObject(Box<models::MessageDeltaContentImageFileObject>),
    #[serde(rename = "image_url")]
    MessageDeltaContentImageUrlObject(Box<models::MessageDeltaContentImageUrlObject>),
    #[serde(rename = "refusal")]
    MessageDeltaContentRefusalObject(Box<models::MessageDeltaContentRefusalObject>),
}'''
            
            content = re.sub(
                r'#\[derive\([^)]*\)\]\s*#\[serde\(tag = "type"\)\]\s*pub enum MessageContentDelta \{\s*\}',
                new_enum,
                content
            )
            
            with open(content_delta_file, 'w') as f:
                f.write(content)
            print(f"Fixed MessageContentDelta enum with proper variants")

def main():
    project_root = Path(__file__).parent.parent
    models_dir = project_root / "src" / "models"
    
    if not models_dir.exists():
        print("No models directory found, skipping fixes")
        return
    
    print("Fixing empty enums in generated Rust code...")
    
    # Apply fixes
    fix_assistant_tool_enum(models_dir)
    fix_chat_completion_message_content_part_enum(models_dir)
    fix_message_delta_content_inner_enum(models_dir)
    fix_message_content_delta_enum(models_dir)
    
    print("\nEmpty enum fixes applied successfully!")

if __name__ == "__main__":
    main()