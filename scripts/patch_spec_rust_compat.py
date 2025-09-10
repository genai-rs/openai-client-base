#!/usr/bin/env python3
"""
Apply Rust compatibility patches to the reconciled OpenAPI spec.

This is LAYER 2 of our patching process - making the spec work with
OpenAPI Generator for Rust after it's been reconciled with the website.

This script handles Rust/generator-specific issues:
1. Simplifying union types that break the generator
2. Fixing multipart form handling
3. Renaming fields that conflict with Rust keywords
4. Removing generator-hostile constructs
"""

import sys, copy, os
import yaml

# This now takes the model fixed spec as input
IN = sys.argv[1] if len(sys.argv) > 1 else 'target/specs/l1_model_fixed.yaml'
OUT = sys.argv[2] if len(sys.argv) > 2 else 'target/specs/l2_rust_compatible.yaml'
REPORT = 'target/reports/l2_rust_compat_report.md'

with open(IN, 'r', encoding='utf-8') as f:
    doc = yaml.safe_load(f)

changes = []
def note(msg):
    changes.append(msg)

def walk_and_prune(node):
    if isinstance(node, dict):
        if 'default' in node: node.pop('default', None); note('Removed default at some nodes')
        node.pop('optional', None)
        node.pop('$recursiveRef', None)
        node.pop('$recursiveAnchor', None)
        node.pop('propertyNames', None)
        for v in node.values():
            walk_and_prune(v)
    elif isinstance(node, list):
        for v in node:
            walk_and_prune(v)

def replace_refs(node, from_ref, to_ref):
    if isinstance(node, dict):
        if node.get('$ref') == from_ref:
            node['$ref'] = to_ref
        for v in node.values():
            replace_refs(v, from_ref, to_ref)
    elif isinstance(node, list):
        for v in node:
            replace_refs(v, from_ref, to_ref)

# Strip any $ref nodes that also define other properties (generator-hostile)
def strip_ref_siblings(node):
    if isinstance(node, dict):
        if '$ref' in node and len(node.keys()) > 1:
            ref = node['$ref']
            # Replace with a pure $ref node
            node.clear()
            node['$ref'] = ref
            note('Stripped $ref siblings on a schema node')
        # Recurse
        for v in node.values():
            strip_ref_siblings(v)
    elif isinstance(node, list):
        for v in node:
            strip_ref_siblings(v)

# 1) Remove all defaults under components.schemas
schemas = doc.get('components', {}).get('schemas', {})
walk_and_prune(schemas)
# Also ensure no $ref siblings across entire document
strip_ref_siblings(doc)

# 2) Simplify union-heavy properties
def set_prop(schema_name, prop, new_schema):
    s = schemas.get(schema_name)
    if s and isinstance(s, dict):
        props = s.setdefault('properties', {})
        if prop in props:
            props[prop] = new_schema
            note(f"Set {schema_name}.{prop} -> {new_schema}")

# Preserve CreateEmbeddingRequest.input union; only set model to string for simplicity
set_prop('CreateEmbeddingRequest', 'model', {'type':'string'})
set_prop('CreateCompletionRequest', 'prompt', {'type':'string'})
set_prop('CreateModerationRequest', 'input', {'type':'string'})
set_prop('RealtimeResponseCreateParams', 'conversation', {'type':'string'})
set_prop('CreateImageEditRequest', 'model', {'type':'string'})
set_prop('CreateImageEditRequest', 'image', {'type':'string'})
set_prop('CreateImageVariationRequest', 'model', {'type':'string'})
set_prop('CreateImageRequest', 'model', {'type':'string'})
set_prop('CreateTranscriptionRequest', 'model', {'type':'string'})
set_prop('CreateTranslationRequest', 'model', {'type':'string'})
# Coerce additional model fields to string for builder usability
set_prop('CreateChatCompletionRequest', 'model', {'type':'string'})
set_prop('CreateModerationRequest', 'model', {'type':'string'})
# In CreateResponse, model lives outside nested object; handle both cases
set_prop('CreateResponse', 'model', {'type':'string'})
# Fix Realtime API model fields to use enums instead of strings
# These need to be kept as references to preserve enum definitions
# set_prop('RealtimeSessionCreateRequest', 'model', {'$ref': '#/components/schemas/RealtimeSessionCreateRequest_model'})
# set_prop('RealtimeTranscriptionSessionCreateRequest', 'model', {'$ref': '#/components/schemas/RealtimeTranscriptionSessionCreateRequest_model'})
# Chat/Message content unions: keep original unions for chat messages to allow enum generation.
# Still simplify non-chat problematic content where needed.
for (schema, prop) in [
    ('CreateMessageRequest','content'),
    ('PredictionContent','content'),
]:
    set_prop(schema, prop, {'type':'string'})

# Responses: simplify CreateResponse.input to string to avoid complex unions in allOf
cr = schemas.get('CreateResponse')
if isinstance(cr, dict) and isinstance(cr.get('allOf'), list):
    for comp in cr['allOf']:
        if isinstance(comp, dict) and comp.get('type') == 'object':
            props = comp.setdefault('properties', {})
            if 'input' in props:
                props['input'] = {'type': 'string'}
            if 'model' in props:
                props['model'] = {'type': 'string'}
set_prop('CreateTranslationRequest', 'model', {'type':'string'})

# 2b) Normalize chat content union variant titles to code-safe names for Rust enums
def normalize_chat_content_titles(schema_name: str):
    s = schemas.get(schema_name)
    if not isinstance(s, dict):
        return
    props = s.get('properties', {})
    c = props.get('content')
    if isinstance(c, dict) and isinstance(c.get('oneOf'), list) and len(c['oneOf']) == 2:
        # First is string → "Text", second is array → "ArrayOfContentParts"
        if isinstance(c['oneOf'][0], dict) and c['oneOf'][0].get('type') == 'string':
            c['oneOf'][0]['title'] = 'Text'
        if isinstance(c['oneOf'][1], dict) and c['oneOf'][1].get('type') == 'array':
            c['oneOf'][1]['title'] = 'ArrayOfContentParts'
        note(f"Normalized {schema_name}.content.oneOf titles -> Text / ArrayOfContentParts")

for name in [
    'ChatCompletionRequestSystemMessage',
    'ChatCompletionRequestUserMessage',
    'ChatCompletionRequestAssistantMessage',
    'ChatCompletionRequestToolMessage',
    'ChatCompletionRequestDeveloperMessage',
]:
    normalize_chat_content_titles(name)

# Normalize CreateEmbeddingRequest.input oneOf titles for Rust enum generation
cer = schemas.get('CreateEmbeddingRequest')
if isinstance(cer, dict):
    props = cer.get('properties', {})
    inp = props.get('input')
    if isinstance(inp, dict) and isinstance(inp.get('oneOf'), list):
        for item in inp['oneOf']:
            if not isinstance(item, dict):
                continue
            t = item.get('type')
            if t == 'string':
                item['title'] = 'Text'
            elif t == 'array':
                it = item.get('items')
                if isinstance(it, dict) and it.get('type') == 'string':
                    item['title'] = 'ArrayOfStrings'
                elif isinstance(it, dict) and it.get('type') == 'integer':
                    item['title'] = 'ArrayOfIntegers'
                elif isinstance(it, dict) and it.get('type') == 'array':
                    inner = it.get('items')
                    if isinstance(inner, dict) and inner.get('type') == 'integer':
                        item['title'] = 'ArrayOfIntegerArrays'
        note('Normalized CreateEmbeddingRequest.input.oneOf titles')

# 3) Coerce problematic event/union models to simple object
for key in ['CreateTranscriptionResponseStreamEvent','OutputItem','RealtimeClientEvent','RealtimeServerEvent','ResponseStreamEvent']:
    if key in schemas:
        schemas[key] = {'type':'object'}
        note(f"Coerced schema {key} -> object")

# 3b) Model id wrapper schemas → simple strings for multipart friendliness
for key in ['CreateTranscriptionRequestModel','CreateTranslationRequestModel','CreateImageVariationRequestModel','CreateEmbeddingRequestModel','CreateImageRequestModel']:
    if key in schemas:
        schemas[key] = {'type':'string'}
for key in ['CreateImageEditRequestModel','CreateImageEditRequestImage']:
    if key in schemas:
        schemas[key] = {'type':'string'}

# 3c) Content union types that generate variant/name clashes → strings
# Do not touch non-existent *Content schemas; we simplified at property level instead

# 4) Rename schema 'Type' to 'TypeAction' and update refs
if 'Type' in schemas:
    schemas['TypeAction'] = schemas.pop('Type')
    replace_refs(doc, '#/components/schemas/Type', '#/components/schemas/TypeAction')
    note('Renamed schema Type -> TypeAction and updated refs')

# Also neutralize ComputerAction
if 'ComputerAction' in schemas:
    schemas['ComputerAction'] = {'type':'object'}

# 5) Normalize query parameters that are objects to strings, remove style/explode
paths = doc.get('paths', {})
for path, item in list(paths.items()):
    # ensure path params present if placeholders exist
    def ensure_path_param(name):
        params = item.setdefault('parameters', [])
        if not any(isinstance(p, dict) and p.get('in')=='path' and p.get('name')==name for p in params):
            params.append({'name': name, 'in': 'path', 'required': True, 'schema': {'type':'string'}})

    if '{certificate_id}' in path:
        ensure_path_param('certificate_id')
    if '{project_id}' in path:
        ensure_path_param('project_id')

    def fix_params_container(container, *, for_path: str):
        if not isinstance(container, list):
            return
        for p in container:
            if not isinstance(p, dict):
                continue
            if p.get('in') == 'query':
                sch = p.get('schema')
                # Metadata ref or object schemas → string
                # Exceptions: allow deepObject for specific known params
                name = p.get('name')
                if for_path == '/fine_tuning/jobs' and name == 'metadata':
                    p['schema'] = {'type': 'object', 'additionalProperties': {'type': 'string'}}
                    p['style'] = 'deepObject'
                    p['explode'] = True
                    note('Restored deepObject for fine_tuning/jobs?metadata')
                    continue
                if for_path == '/organization/audit_logs' and name == 'effective_at':
                    p['schema'] = {
                        'type': 'object',
                        'properties': {
                            'gt': {'type': 'integer'},
                            'gte': {'type': 'integer'},
                            'lt': {'type': 'integer'},
                            'lte': {'type': 'integer'},
                        }
                    }
                    p['style'] = 'deepObject'
                    p['explode'] = True
                    note('Restored deepObject for organization/audit_logs?effective_at')
                    continue
                if (isinstance(sch, dict) and (sch.get('type') == 'object' or sch.get('$ref') == '#/components/schemas/Metadata')):
                    p['schema'] = {'type':'string'}
                    note(f"Coerced query param {p.get('name')} at {path} -> string")
                # cleanup
                p.pop('style', None)
                p.pop('explode', None)

    # path-level params
    fix_params_container(item.get('parameters'), for_path=path)
    # op-level params
    for method in ['get','post','put','patch','delete','options','head']:
        op = item.get(method)
        if isinstance(op, dict):
            fix_params_container(op.get('parameters'), for_path=path)
            # Rename mismatched cert_id -> certificate_id to match path placeholder
            if '{certificate_id}' in path and isinstance(op.get('parameters'), list):
                for p in op['parameters']:
                    if isinstance(p, dict) and p.get('in') == 'path' and p.get('name') == 'cert_id':
                        p['name'] = 'certificate_id'
                        note(f"Renamed op path param cert_id -> certificate_id at {path}")

with open(OUT, 'w', encoding='utf-8') as f:
    yaml.safe_dump(doc, f, sort_keys=False)

print(f"Patched spec written to {OUT}")

# Write report
with open(REPORT, 'w', encoding='utf-8') as rf:
    rf.write('# Patch Report\n\n')
    rf.write(f'- Source: {os.path.relpath(IN)}\n')
    rf.write(f'- Output: {os.path.relpath(OUT)}\n')
    rf.write('\n## Changes\n')
    if not changes:
        rf.write('- No structural changes recorded.\n')
    else:
        seen = set()
        for c in changes:
            if c not in seen:
                rf.write(f'- {c}\n')
                seen.add(c)
