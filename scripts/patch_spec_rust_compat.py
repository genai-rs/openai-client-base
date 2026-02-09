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

import sys, copy, os, re
import yaml

# This now takes the model fixed spec as input
IN = sys.argv[1] if len(sys.argv) > 1 else "target/specs/l1_model_fixed.yaml"
OUT = sys.argv[2] if len(sys.argv) > 2 else "target/specs/l2_rust_compatible.yaml"
REPORT = "target/reports/l2_rust_compat_report.md"

with open(IN, "r", encoding="utf-8") as f:
    doc = yaml.safe_load(f)

changes = []


def note(msg):
    changes.append(msg)


def walk_and_prune(node):
    if isinstance(node, dict):
        if "default" in node:
            node.pop("default", None)
            note("Removed default at some nodes")
        node.pop("optional", None)
        node.pop("$recursiveRef", None)
        node.pop("$recursiveAnchor", None)
        node.pop("propertyNames", None)
        for v in node.values():
            walk_and_prune(v)
    elif isinstance(node, list):
        for v in node:
            walk_and_prune(v)


def replace_refs(node, from_ref, to_ref):
    if isinstance(node, dict):
        if node.get("$ref") == from_ref:
            node["$ref"] = to_ref
        for v in node.values():
            replace_refs(v, from_ref, to_ref)
    elif isinstance(node, list):
        for v in node:
            replace_refs(v, from_ref, to_ref)


# Strip any $ref nodes that also define other properties (generator-hostile)
def strip_ref_siblings(node):
    if isinstance(node, dict):
        if "$ref" in node and len(node.keys()) > 1:
            ref = node["$ref"]
            # Replace with a pure $ref node
            node.clear()
            node["$ref"] = ref
            note("Stripped $ref siblings on a schema node")
        # Recurse
        for v in node.values():
            strip_ref_siblings(v)
    elif isinstance(node, list):
        for v in node:
            strip_ref_siblings(v)


def collapse_required_only_unions(node):
    """Remove oneOf/anyOf where every item is just a {required: [...]} constraint.

    These are pure validation constraints (e.g. "provide vector_store_ids OR
    vector_stores") with no type/properties/$ref/title, so openapi-generator
    cannot derive a model name for them and throws a NullPointerException.
    Removing the keyword leaves the parent schema intact (with its own type,
    properties, etc.) -- the validation-only union is simply dropped.
    """
    if isinstance(node, dict):
        for keyword in ("oneOf", "anyOf"):
            items = node.get(keyword)
            if isinstance(items, list) and len(items) > 0:
                # Keys that indicate an item represents a real, distinct type
                type_keys = {"type", "properties", "$ref", "title"}
                all_required_only = all(
                    isinstance(item, dict) and not (set(item.keys()) & type_keys)
                    for item in items
                )
                if all_required_only:
                    del node[keyword]
                    note(f"Collapsed validation-only {keyword} ({len(items)} items)")
        for v in node.values():
            collapse_required_only_unions(v)
    elif isinstance(node, list):
        for v in node:
            collapse_required_only_unions(v)


def add_titles_to_unnamed_union_variants(node, path=""):
    """Add titles to oneOf/anyOf variants that have a type but no title.

    openapi-generator needs titles to derive Rust enum variant names.
    Without titles, it passes null to sanitizeIdentifier() and crashes
    with a NullPointerException.
    """
    if isinstance(node, dict):
        for keyword in ("oneOf", "anyOf"):
            items = node.get(keyword)
            if isinstance(items, list):
                for i, item in enumerate(items):
                    if (
                        isinstance(item, dict)
                        and "title" not in item
                        and "$ref" not in item
                    ):
                        t = item.get("type")
                        if t == "string":
                            item["title"] = "Text"
                        elif t == "integer":
                            item["title"] = "Integer"
                        elif t == "number":
                            item["title"] = "Number"
                        elif t == "boolean":
                            item["title"] = "Boolean"
                        elif t == "array":
                            items_schema = item.get("items", {})
                            inner_type = items_schema.get("type", "item").capitalize()
                            item["title"] = f"ArrayOf{inner_type}s"
                        elif t == "object":
                            item["title"] = f"Object{i}"
                        if "title" in item:
                            note(
                                f"Auto-titled {keyword}[{i}] as '{item['title']}' at {path}"
                            )
        for k, v in node.items():
            add_titles_to_unnamed_union_variants(v, path=f"{path}.{k}")
    elif isinstance(node, list):
        for i, v in enumerate(node):
            add_titles_to_unnamed_union_variants(v, path=f"{path}[{i}]")


def hoist_inline_unions(schemas):
    """Extract inline oneOf/anyOf from schema properties into named top-level schemas.

    When a property contains an inline oneOf/anyOf where ALL variants are non-$ref
    (i.e. primitive or inline types), openapi-generator's flattening creates a derived
    model name like ``SchemaName_propertyName`` but then fails with a
    NullPointerException because the individual variants lack names.

    Adding titles to the variants is not sufficient -- the generator still crashes
    when processing the flattened composed model.  The robust fix is to hoist the
    inline union into a named top-level schema and replace the property with a ``$ref``.
    """
    new_schemas = {}

    for schema_name, schema in list(schemas.items()):
        if not isinstance(schema, dict):
            continue
        props = schema.get("properties")
        if not isinstance(props, dict):
            continue

        for prop_name, prop_schema in list(props.items()):
            if not isinstance(prop_schema, dict):
                continue

            for keyword in ("oneOf", "anyOf"):
                items = prop_schema.get(keyword)
                if not isinstance(items, list) or len(items) == 0:
                    continue

                # Only hoist when every variant is an inline type (no $ref).
                # Unions that mix $ref and inline types work fine with titles.
                has_ref = any(
                    isinstance(item, dict) and "$ref" in item for item in items
                )
                if has_ref:
                    continue

                # Skip nullable wrappers: anyOf/oneOf [SomeType, {type: null}]
                # These are just Option<T> and the generator handles them fine inline.
                has_null_variant = any(
                    isinstance(item, dict) and item.get("type") == "null"
                    for item in items
                )
                if has_null_variant:
                    continue

                # Only hoist unions containing at least one complex type (array
                # or object).  Simple unions like string_enum("auto") | number
                # work fine inline with titles -- hoisting them creates named
                # schemas the generator doesn't produce model files for.
                complex_types = {"array", "object"}
                has_complex = any(
                    isinstance(item, dict) and item.get("type") in complex_types
                    for item in items
                )
                if not has_complex:
                    continue

                # Build a CamelCase name: SchemaName + PropertyName
                # Convert snake_case property names to CamelCase
                camel_prop = "".join(part.capitalize() for part in prop_name.split("_"))
                new_name = f"{schema_name}{camel_prop}"

                # Build the new top-level schema
                hoisted = {keyword: items, "title": new_name}
                desc = prop_schema.get("description")
                if desc:
                    hoisted["description"] = desc

                new_schemas[new_name] = hoisted

                # Replace the inline union with a plain $ref.
                # Description is already on the hoisted schema.
                props[prop_name] = {"$ref": f"#/components/schemas/{new_name}"}
                note(
                    f"Hoisted inline {keyword} {schema_name}.{prop_name} -> {new_name}"
                )
                break  # only one keyword per property

    schemas.update(new_schemas)


# 1) Remove all defaults under components.schemas
schemas = doc.get("components", {}).get("schemas", {})
walk_and_prune(schemas)
# Also ensure no $ref siblings across entire document
strip_ref_siblings(doc)
# Remove oneOf/anyOf entries that only express required-field constraints
collapse_required_only_unions(doc)
# Auto-title unnamed union variants so the generator can derive model names
add_titles_to_unnamed_union_variants(doc)
# Hoist inline unions (all-primitive oneOf/anyOf) from properties into named schemas
hoist_inline_unions(schemas)


# 2) Simplify union-heavy properties
def set_prop(schema_name, prop, new_schema):
    s = schemas.get(schema_name)
    if s and isinstance(s, dict):
        props = s.setdefault("properties", {})
        if prop in props:
            props[prop] = new_schema
            note(f"Set {schema_name}.{prop} -> {new_schema}")


# Preserve CreateEmbeddingRequest.input union; only set model to string for simplicity
set_prop("CreateEmbeddingRequest", "model", {"type": "string"})
set_prop("CreateCompletionRequest", "prompt", {"type": "string"})
set_prop("CreateModerationRequest", "input", {"type": "string"})
set_prop("RealtimeResponseCreateParams", "conversation", {"type": "string"})
set_prop("CreateImageEditRequest", "model", {"type": "string"})
set_prop("CreateImageEditRequest", "image", {"type": "string"})
set_prop("CreateImageVariationRequest", "model", {"type": "string"})
set_prop("CreateImageRequest", "model", {"type": "string"})
set_prop("CreateTranscriptionRequest", "model", {"type": "string"})
set_prop("CreateTranslationRequest", "model", {"type": "string"})
# Coerce additional model fields to string for builder usability
set_prop("CreateChatCompletionRequest", "model", {"type": "string"})
set_prop("CreateModerationRequest", "model", {"type": "string"})
# In CreateResponse, model lives outside nested object; handle both cases
set_prop("CreateResponse", "model", {"type": "string"})
# Fix Realtime API model fields to use enums instead of strings
# These need to be kept as references to preserve enum definitions
# set_prop('RealtimeSessionCreateRequest', 'model', {'$ref': '#/components/schemas/RealtimeSessionCreateRequest_model'})
# set_prop('RealtimeTranscriptionSessionCreateRequest', 'model', {'$ref': '#/components/schemas/RealtimeTranscriptionSessionCreateRequest_model'})
# Chat/Message content unions: keep original unions for chat messages to allow enum generation.
# Still simplify non-chat problematic content where needed.
for schema, prop in [
    ("CreateMessageRequest", "content"),
    ("PredictionContent", "content"),
]:
    set_prop(schema, prop, {"type": "string"})

# Responses: simplify CreateResponse.input to string to avoid complex unions in allOf
cr = schemas.get("CreateResponse")
if isinstance(cr, dict) and isinstance(cr.get("allOf"), list):
    for comp in cr["allOf"]:
        if isinstance(comp, dict) and comp.get("type") == "object":
            props = comp.setdefault("properties", {})
            if "input" in props:
                props["input"] = {"type": "string"}
            if "model" in props:
                props["model"] = {"type": "string"}
set_prop("CreateTranslationRequest", "model", {"type": "string"})

# Response schema also uses allOf; simplify instructions to string
# The spec has a broken nested anyOf-inside-anyOf that produces an empty enum
resp = schemas.get("Response")
if isinstance(resp, dict) and isinstance(resp.get("allOf"), list):
    for comp in resp["allOf"]:
        if isinstance(comp, dict) and comp.get("type") == "object":
            props = comp.setdefault("properties", {})
            if "instructions" in props:
                props["instructions"] = {"type": "string"}
                note("Simplified Response.instructions -> string")

# 3) Coerce problematic event/union models to simple object
for key in [
    "CreateTranscriptionResponseStreamEvent",
    "OutputItem",
    "RealtimeClientEvent",
    "RealtimeServerEvent",
    "ResponseStreamEvent",
]:
    if key in schemas:
        schemas[key] = {"type": "object"}
        note(f"Coerced schema {key} -> object")

# 3b) Model id wrapper schemas → simple strings for multipart friendliness
for key in [
    "CreateTranscriptionRequestModel",
    "CreateTranslationRequestModel",
    "CreateImageVariationRequestModel",
    "CreateEmbeddingRequestModel",
    "CreateImageRequestModel",
]:
    if key in schemas:
        schemas[key] = {"type": "string"}
for key in ["CreateImageEditRequestModel", "CreateImageEditRequestImage"]:
    if key in schemas:
        schemas[key] = {"type": "string"}

# 3c) Content union types that generate variant/name clashes → strings
# Do not touch non-existent *Content schemas; we simplified at property level instead

# 4) Rename schema 'Type' to 'TypeAction' and update refs
if "Type" in schemas:
    schemas["TypeAction"] = schemas.pop("Type")
    replace_refs(doc, "#/components/schemas/Type", "#/components/schemas/TypeAction")
    note("Renamed schema Type -> TypeAction and updated refs")

# 4b) Normalize schema names with hyphens to valid Rust identifiers
# Find all schemas with hyphens and rename them
hyphenated_schemas = [name for name in schemas.keys() if "-" in name]
for old_name in hyphenated_schemas:
    # Replace hyphens with underscores or remove them for numbers
    # e.g., "ConversationParam-2" -> "ConversationParam2"
    new_name = re.sub(r"-(\d+)", r"\1", old_name)  # Remove hyphen before digits
    new_name = new_name.replace("-", "_")  # Replace remaining hyphens with underscores

    if new_name != old_name:
        schemas[new_name] = schemas.pop(old_name)
        old_ref = f"#/components/schemas/{old_name}"
        new_ref = f"#/components/schemas/{new_name}"
        replace_refs(doc, old_ref, new_ref)
        note(f"Normalized schema name: {old_name} -> {new_name}")

# Also neutralize ComputerAction
if "ComputerAction" in schemas:
    schemas["ComputerAction"] = {"type": "object"}

# 5) Normalize query parameters that are objects to strings, remove style/explode
paths = doc.get("paths", {})
for path, item in list(paths.items()):
    # ensure path params present if placeholders exist
    def ensure_path_param(name):
        params = item.setdefault("parameters", [])
        if not any(
            isinstance(p, dict) and p.get("in") == "path" and p.get("name") == name
            for p in params
        ):
            params.append(
                {
                    "name": name,
                    "in": "path",
                    "required": True,
                    "schema": {"type": "string"},
                }
            )

    if "{certificate_id}" in path:
        ensure_path_param("certificate_id")
    if "{project_id}" in path:
        ensure_path_param("project_id")

    def fix_params_container(container, *, for_path: str):
        if not isinstance(container, list):
            return
        for p in container:
            if not isinstance(p, dict):
                continue
            if p.get("in") == "query":
                sch = p.get("schema")
                # Metadata ref or object schemas → string
                # Exceptions: allow deepObject for specific known params
                name = p.get("name")
                if for_path == "/fine_tuning/jobs" and name == "metadata":
                    p["schema"] = {
                        "type": "object",
                        "additionalProperties": {"type": "string"},
                    }
                    p["style"] = "deepObject"
                    p["explode"] = True
                    note("Restored deepObject for fine_tuning/jobs?metadata")
                    continue
                if for_path == "/organization/audit_logs" and name == "effective_at":
                    p["schema"] = {
                        "type": "object",
                        "properties": {
                            "gt": {"type": "integer"},
                            "gte": {"type": "integer"},
                            "lt": {"type": "integer"},
                            "lte": {"type": "integer"},
                        },
                    }
                    p["style"] = "deepObject"
                    p["explode"] = True
                    note("Restored deepObject for organization/audit_logs?effective_at")
                    continue
                if isinstance(sch, dict) and (
                    sch.get("type") == "object"
                    or sch.get("$ref") == "#/components/schemas/Metadata"
                ):
                    p["schema"] = {"type": "string"}
                    note(f"Coerced query param {p.get('name')} at {path} -> string")
                # cleanup
                p.pop("style", None)
                p.pop("explode", None)

    # path-level params
    fix_params_container(item.get("parameters"), for_path=path)
    # op-level params
    for method in ["get", "post", "put", "patch", "delete", "options", "head"]:
        op = item.get(method)
        if isinstance(op, dict):
            fix_params_container(op.get("parameters"), for_path=path)
            # Rename mismatched cert_id -> certificate_id to match path placeholder
            if "{certificate_id}" in path and isinstance(op.get("parameters"), list):
                for p in op["parameters"]:
                    if (
                        isinstance(p, dict)
                        and p.get("in") == "path"
                        and p.get("name") == "cert_id"
                    ):
                        p["name"] = "certificate_id"
                        note(
                            f"Renamed op path param cert_id -> certificate_id at {path}"
                        )

with open(OUT, "w", encoding="utf-8") as f:
    yaml.safe_dump(doc, f, sort_keys=False)

print(f"Patched spec written to {OUT}")

# Write report
with open(REPORT, "w", encoding="utf-8") as rf:
    rf.write("# Patch Report\n\n")
    rf.write(f"- Source: {os.path.relpath(IN)}\n")
    rf.write(f"- Output: {os.path.relpath(OUT)}\n")
    rf.write("\n## Changes\n")
    if not changes:
        rf.write("- No structural changes recorded.\n")
    else:
        seen = set()
        for c in changes:
            if c not in seen:
                rf.write(f"- {c}\n")
                seen.add(c)
