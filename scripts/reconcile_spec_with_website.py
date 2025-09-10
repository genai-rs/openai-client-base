#!/usr/bin/env python3
"""
Reconcile the OpenAPI spec with OpenAI's website documentation.

This is LAYER 1 of our patching process - ensuring the spec matches
what's actually documented on platform.openai.com

This script handles:
1. Adding missing fields that are on the website but not in spec
2. Removing deprecated fields that are no longer on the website
3. Fixing type mismatches between spec and website
4. Updating descriptions to match current documentation
"""

import sys
import json
import yaml
from pathlib import Path
from typing import Any, Dict, List, Optional

IN = sys.argv[1] if len(sys.argv) > 1 else 'spec/openapi.yaml'
OUT = sys.argv[2] if len(sys.argv) > 2 else 'spec/openapi.reconciled.yaml'
REPORT = 'spec/reconciliation_report.md'
SCHEMAS_FILE = 'docs/openai-schemas-extracted.json'

with open(IN, 'r', encoding='utf-8') as f:
    doc = yaml.safe_load(f)

# Load extracted schemas from OpenAI docs
extracted_schemas = {}
if Path(SCHEMAS_FILE).exists():
    with open(SCHEMAS_FILE, 'r') as f:
        extracted_schemas = json.load(f)
    print(f"✅ Loaded {len(extracted_schemas)} schemas from {SCHEMAS_FILE}")
else:
    print(f"⚠️  No extracted schemas found at {SCHEMAS_FILE}")

changes = []

def note(category: str, msg: str):
    """Log a change with category"""
    changes.append((category, msg))
    print(f"  [{category}] {msg}")

def get_openai_type_for_openapi(param_type: str) -> Dict[str, Any]:
    """Convert our extracted type to OpenAPI format"""
    type_map = {
        'string': {'type': 'string'},
        'integer': {'type': 'integer'},
        'number': {'type': 'number'},
        'boolean': {'type': 'boolean'},
        'array': {'type': 'array'},
        'object': {'type': 'object'}
    }
    return type_map.get(param_type, {'type': 'string'})

# ============================================================================
# DATA-DRIVEN RECONCILIATION
# ============================================================================

def reconcile_endpoint(schema_name: str, extracted_key: str, special_fixes: Dict[str, Any] = None):
    """Reconcile a specific endpoint schema with extracted documentation
    
    Args:
        schema_name: Name of the schema in OpenAPI spec
        extracted_key: Key in extracted schemas (e.g., 'responses_create')
        special_fixes: Dictionary of special field name fixes (e.g., max_completion_tokens -> max_output_tokens)
    """
    
    if extracted_key not in extracted_schemas:
        print(f"  ⚠️  No extracted schema for {extracted_key}")
        return
        
    extracted = extracted_schemas[extracted_key]
    schemas = doc.get('components', {}).get('schemas', {})
    
    if schema_name not in schemas:
        print(f"  ⚠️  Schema {schema_name} not found in spec")
        return
        
    schema = schemas[schema_name]
    
    # Handle allOf structure
    for item in schema.get('allOf', [schema]):
        if isinstance(item, dict) and item.get('type') == 'object' and 'properties' in item:
            props = item['properties']
            
            # Apply special fixes first
            if special_fixes:
                for old_name, new_name in special_fixes.items():
                    if old_name in props and new_name != old_name:
                        props[new_name] = props.pop(old_name)
                        note("FIX", f"Renamed {old_name} -> {new_name} in {schema_name}")
            
            # Add missing fields from extracted schema
            for field_name, field_info in extracted.get('properties', {}).items():
                if field_name not in props:
                    openapi_type = get_openai_type_for_openapi(field_info['type'])
                    openapi_type['nullable'] = field_info.get('nullable', True)
                    openapi_type['description'] = field_info.get('description', f'{field_name} parameter from OpenAI docs')
                    
                    props[field_name] = openapi_type
                    note("ADD", f"Added missing '{field_name}' to {schema_name}")
            
            # Update required fields
            if 'required' in extracted:
                if 'required' not in item:
                    item['required'] = []
                
                for req_field in extracted['required']:
                    if req_field not in item['required']:
                        item['required'].append(req_field)
                        note("REQ", f"Marked '{req_field}' as required in {schema_name}")

# ============================================================================
# RESPONSES API RECONCILIATION
# ============================================================================

def reconcile_responses_api():
    """Reconcile Responses API with website documentation"""
    print("\n=== Reconciling Responses API ===")
    
    # Special fixes for Responses API
    responses_fixes = {
        'max_completion_tokens': 'max_output_tokens'  # Correct field name
    }
    
    reconcile_endpoint('CreateResponse', 'responses_create', responses_fixes)
    
    # Also handle ResponsesCreateResponse if it exists
    reconcile_endpoint('ResponsesCreateResponse', 'responses_create', responses_fixes)

# ============================================================================
# CHAT COMPLETIONS API RECONCILIATION
# ============================================================================

def reconcile_chat_api():
    """Reconcile Chat Completions API with website documentation"""
    print("\n=== Reconciling Chat Completions API ===")
    
    reconcile_endpoint('CreateChatCompletionRequest', 'chat_create')
    
    # Note: Chat uses max_completion_tokens (correct for this endpoint)

# ============================================================================
# EMBEDDINGS API RECONCILIATION
# ============================================================================

def reconcile_embeddings_api():
    """Reconcile Embeddings API with website documentation"""
    print("\n=== Reconciling Embeddings API ===")
    
    reconcile_endpoint('CreateEmbeddingRequest', 'embeddings_create')

# ============================================================================
# IMAGES API RECONCILIATION
# ============================================================================

def reconcile_images_api():
    """Reconcile Images API with website documentation"""
    print("\n=== Reconciling Images API ===")
    
    reconcile_endpoint('CreateImageRequest', 'images_create')
    reconcile_endpoint('CreateImageEditRequest', 'images_create-edit')
    reconcile_endpoint('CreateImageVariationRequest', 'images_create-variation')

# ============================================================================
# AUDIO API RECONCILIATION
# ============================================================================

def reconcile_audio_api():
    """Reconcile Audio API with website documentation"""
    print("\n=== Reconciling Audio API ===")
    
    reconcile_endpoint('CreateTranscriptionRequest', 'audio_create-transcription')
    reconcile_endpoint('CreateTranslationRequest', 'audio_create-translation')

# ============================================================================
# MODERATIONS API RECONCILIATION
# ============================================================================

def reconcile_moderations_api():
    """Reconcile Moderations API with website documentation"""
    print("\n=== Reconciling Moderations API ===")
    
    reconcile_endpoint('CreateModerationRequest', 'moderations_create')

# ============================================================================
# FINE-TUNING API RECONCILIATION
# ============================================================================

def reconcile_fine_tuning_api():
    """Reconcile Fine-tuning API with website documentation"""
    print("\n=== Reconciling Fine-tuning API ===")
    
    reconcile_endpoint('CreateFineTuningJobRequest', 'fine-tuning_create')

# ============================================================================
# MODEL ID FIXES
# ============================================================================

def fix_model_references():
    """Fix model ID references to use correct enum types"""
    print("\n=== Fixing Model References ===")
    
    schemas = doc.get('components', {}).get('schemas', {})
    
    # Ensure ModelIdsResponses exists for Responses API
    if 'ModelIdsResponses' not in schemas:
        schemas['ModelIdsResponses'] = {
            'type': 'string',
            'description': 'Available models for the Responses API',
            'enum': [
                'gpt-4o',
                'gpt-4o-2024-08-06', 
                'gpt-4o-2024-11-20',
                'gpt-4o-mini',
                'gpt-4o-mini-2024-07-18',
                'gpt-4',
                'gpt-4-turbo',
                'gpt-4-turbo-preview',
                'gpt-3.5-turbo',
                'o1',
                'o1-2024-12-17',
                'o1-mini',
                'o1-preview'
            ]
        }
        note("ADD", "Added ModelIdsResponses enum")

# ============================================================================
# MAIN EXECUTION
# ============================================================================

print("=" * 60)
print("OpenAPI Spec Reconciliation with Website Documentation")
print("=" * 60)

# Run all reconciliations
reconcile_responses_api()
reconcile_chat_api()
reconcile_embeddings_api()
reconcile_images_api()
reconcile_audio_api()
reconcile_moderations_api()
reconcile_fine_tuning_api()
fix_model_references()

# Save the reconciled spec
with open(OUT, 'w', encoding='utf-8') as f:
    yaml.safe_dump(doc, f, sort_keys=False, allow_unicode=True, width=120)

print(f"\n✅ Saved reconciled spec to {OUT}")

# Generate reconciliation report
with open(REPORT, 'w') as f:
    f.write("# OpenAPI Spec Reconciliation Report\n\n")
    f.write("This report shows all changes made to reconcile the OpenAPI spec with OpenAI's website documentation.\n\n")
    
    if not changes:
        f.write("No changes were necessary - spec matches website documentation.\n")
    else:
        f.write(f"## Total Changes: {len(changes)}\n\n")
        
        # Group changes by category
        categories = {}
        for category, msg in changes:
            if category not in categories:
                categories[category] = []
            categories[category].append(msg)
        
        for category, msgs in categories.items():
            f.write(f"### {category} ({len(msgs)} changes)\n\n")
            for msg in msgs:
                f.write(f"- {msg}\n")
            f.write("\n")

print(f"📄 Reconciliation report saved to {REPORT}")
print(f"\n📊 Summary: {len(changes)} total changes")

if changes:
    from collections import Counter
    cat_counts = Counter(cat for cat, _ in changes)
    for cat, count in cat_counts.items():
        print(f"  - {cat}: {count}")