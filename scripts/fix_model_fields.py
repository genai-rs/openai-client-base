#!/usr/bin/env python3
"""
Fix model fields in OpenAPI spec to be simple strings instead of complex references.

This script specifically handles the model field issue where:
1. Model fields are referenced through allOf inheritance
2. They point to complex enum schemas that generate as empty structs
3. We need them to be simple string types for ergonomic Rust usage

This runs AFTER reconciliation but BEFORE the main Rust compatibility patch.
"""

import sys
import os
from typing import Dict, Any, Set
from utils import load_spec, save_spec

def find_model_fields_in_allof(spec: Dict[str, Any]) -> Set[str]:
    """Find all schemas that have model fields inherited through allOf."""
    schemas = spec.get('components', {}).get('schemas', {})
    model_schemas = set()
    
    for schema_name, schema in schemas.items():
        if 'allOf' in schema:
            # Check if any of the allOf references contain model fields
            for ref_item in schema.get('allOf', []):
                if '$ref' in ref_item:
                    ref_path = ref_item['$ref'].split('/')[-1]
                    # Check if the referenced schema has a model property
                    ref_schema = schemas.get(ref_path, {})
                    if 'properties' in ref_schema and 'model' in ref_schema.get('properties', {}):
                        model_schemas.add(schema_name)
                        print(f"Found {schema_name} inherits model from {ref_path}")
    
    return model_schemas

def flatten_model_fields(spec: Dict[str, Any]) -> Dict[str, Any]:
    """
    Flatten model field definitions to be simple strings.
    
    This handles three cases:
    1. Direct model properties that reference complex schemas
    2. Model properties inherited through allOf
    3. Model wrapper schemas (ModelIds*) that should be simple strings
    """
    schemas = spec.get('components', {}).get('schemas', {})
    changes = []
    
    # Step 1: Replace all ModelIds* and VoiceIds* schemas with simple string schemas
    model_id_schemas = [
        'ModelIds',
        'ModelIdsShared', 
        'ModelIdsResponses',
        'CreateCompletionRequestModel',
        'CreateAssistantRequestModel',
        'CreateThreadAndRunRequestModel',
        'CreateImageRequestModel',
        'CreateEmbeddingRequestModel',
        'CreateSpeechRequestModel',
        'CreateFineTuningJobRequestModel',
        'CreateTranscriptionRequestModel',
        'CreateTranslationRequestModel',
        'CreateModerationRequestModel',
        'CreateImageEditRequestModel',
        'CreateImageVariationRequestModel',
        'VoiceIdsShared'
    ]
    
    for schema_name in model_id_schemas:
        if schema_name in schemas:
            old_schema = schemas[schema_name]
            # Replace with a simple string schema
            schemas[schema_name] = {
                'type': 'string',
                'description': old_schema.get('description', f'Model identifier as string')
            }
            changes.append(f"Replaced {schema_name} with simple string type")
    
    # Step 2: For schemas with allOf, inject model field directly if inherited
    for schema_name, schema in schemas.items():
        if 'allOf' in schema:
            has_model = False
            # Check each allOf component
            for ref_item in schema.get('allOf', []):
                if '$ref' in ref_item:
                    ref_name = ref_item['$ref'].split('/')[-1]
                    ref_schema = schemas.get(ref_name, {})
                    if 'properties' in ref_schema and 'model' in ref_schema.get('properties', {}):
                        has_model = True
                        break
            
            if has_model:
                # Find or create the properties object in the allOf
                for item in schema['allOf']:
                    if 'type' in item and item['type'] == 'object':
                        if 'properties' not in item:
                            item['properties'] = {}
                        # Add model as a simple string
                        item['properties']['model'] = {
                            'type': 'string',
                            'description': 'ID of the model to use'
                        }
                        changes.append(f"Injected model field into {schema_name}")
                        break
    
    # Step 3: Fix any direct model properties that still reference complex types
    for schema_name, schema in schemas.items():
        if 'properties' in schema:
            props = schema['properties']
            if 'model' in props:
                model_prop = props['model']
                # Check if it's a reference to a complex type
                if '$ref' in model_prop:
                    ref_name = model_prop['$ref'].split('/')[-1]
                    if ref_name in model_id_schemas:
                        props['model'] = {
                            'type': 'string',
                            'description': model_prop.get('description', 'ID of the model to use')
                        }
                        changes.append(f"Converted {schema_name}.model from $ref to string")
                elif 'allOf' in model_prop or 'oneOf' in model_prop or 'anyOf' in model_prop:
                    # Complex union type - simplify to string
                    props['model'] = {
                        'type': 'string', 
                        'description': model_prop.get('description', 'ID of the model to use')
                    }
                    changes.append(f"Simplified {schema_name}.model from union to string")
    
    # Step 4: Fix any voice properties that reference VoiceIdsShared
    for schema_name, schema in schemas.items():
        if 'properties' in schema:
            props = schema['properties']
            if 'voice' in props:
                voice_prop = props['voice']
                # Check if it's a reference to VoiceIdsShared
                if '$ref' in voice_prop:
                    ref_name = voice_prop['$ref'].split('/')[-1]
                    if ref_name == 'VoiceIdsShared':
                        props['voice'] = {
                            'type': 'string',
                            'description': voice_prop.get('description', 'The voice to use for audio generation')
                        }
                        changes.append(f"Converted {schema_name}.voice from $ref to string")
                elif 'allOf' in voice_prop or 'oneOf' in voice_prop or 'anyOf' in voice_prop:
                    # Complex union type - simplify to string
                    props['voice'] = {
                        'type': 'string', 
                        'description': voice_prop.get('description', 'The voice to use for audio generation')
                    }
                    changes.append(f"Simplified {schema_name}.voice from union to string")
    
    # Step 5: Handle ModelResponseProperties which contains the model field
    if 'ModelResponseProperties' in schemas:
        model_props = schemas['ModelResponseProperties']
        if 'properties' in model_props and 'model' in model_props['properties']:
            model_props['properties']['model'] = {
                'type': 'string',
                'description': 'ID of the model to use'
            }
            changes.append("Fixed ModelResponseProperties.model to be string")
    
    return changes

def main():
    # Input and output paths
    input_path = sys.argv[1] if len(sys.argv) > 1 else 'stainless.yaml'
    output_path = sys.argv[2] if len(sys.argv) > 2 else 'target/specs/l1_model_fixed.yaml'
    
    print(f"Loading spec from {input_path}")
    spec = load_spec(input_path)
    
    print("\nFinding model fields in allOf inheritance...")
    model_schemas = find_model_fields_in_allof(spec)
    print(f"Found {len(model_schemas)} schemas with inherited model fields")
    
    print("\nFlattening model field definitions...")
    changes = flatten_model_fields(spec)
    
    print("\nChanges made:")
    for change in changes:
        print(f"  - {change}")
    
    print(f"\nSaving fixed spec to {output_path}")
    save_spec(spec, output_path)
    
    # Write a report
    report_path = 'target/reports/l1_model_fields_fix_report.md'
    with open(report_path, 'w') as f:
        f.write("# Model Fields Fix Report\n\n")
        f.write(f"- Input: {os.path.relpath(input_path)}\n")
        f.write(f"- Output: {os.path.relpath(output_path)}\n")
        f.write(f"- Total changes: {len(changes)}\n\n")
        f.write("## Changes Applied\n\n")
        for change in changes:
            f.write(f"- {change}\n")
    
    print(f"Report saved to {report_path}")
    print(f"\nTotal changes: {len(changes)}")

if __name__ == "__main__":
    main()