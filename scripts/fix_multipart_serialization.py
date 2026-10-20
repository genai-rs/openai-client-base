#!/usr/bin/env python3
"""Apply schema-scoped multipart array and structured-value serialization fixes."""

from __future__ import annotations

import argparse
import copy
import json
import re
from pathlib import Path
from typing import Any

import yaml


HTTP_METHODS = {"get", "post", "put", "patch", "delete"}
SCALAR_TYPES = {"string", "integer", "number", "boolean"}


def local_ref(spec: dict[str, Any], reference: str) -> Any:
    if not reference.startswith("#/"):
        raise ValueError(f"unsupported external OpenAPI reference: {reference}")
    current: Any = spec
    for raw_part in reference[2:].split("/"):
        part = raw_part.replace("~1", "/").replace("~0", "~")
        if not isinstance(current, dict) or part not in current:
            raise ValueError(f"unresolved OpenAPI reference: {reference}")
        current = current[part]
    return current


def resolve(spec: dict[str, Any], value: Any, stack: tuple[str, ...] = ()) -> Any:
    if isinstance(value, list):
        return [resolve(spec, item, stack) for item in value]
    if not isinstance(value, dict):
        return value
    reference = value.get("$ref")
    if reference is None:
        return {key: resolve(spec, item, stack) for key, item in value.items()}
    if reference in stack:
        # Recursive schemas are valid OpenAPI (for example, nested filters).
        # Preserve the recursive edge: the multipart decisions only need the
        # finite root/field shape and must not expand an entire model graph.
        return copy.deepcopy(value)
    resolved = resolve(spec, local_ref(spec, reference), stack + (reference,))
    if not isinstance(resolved, dict):
        return resolved
    siblings = {key: resolve(spec, item, stack) for key, item in value.items() if key != "$ref"}
    return {**resolved, **siblings}


def resolve_root(spec: dict[str, Any], value: Any, context: str) -> Any:
    """Resolve a single local Reference Object without expanding nested schemas."""
    seen: set[str] = set()
    current = value
    while isinstance(current, dict) and "$ref" in current:
        reference = current["$ref"]
        if reference in seen:
            raise ValueError(f"cyclic OpenAPI reference for {context}: {reference}")
        seen.add(reference)
        resolved = local_ref(spec, reference)
        if not isinstance(resolved, dict):
            raise ValueError(f"OpenAPI reference is not an object for {context}: {reference}")
        siblings = {key: item for key, item in current.items() if key != "$ref"}
        current = {**resolved, **siblings}
    return current


def operation_name(operation_id: str) -> str:
    name = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", operation_id)
    name = re.sub(r"[^A-Za-z0-9]+", "_", name).strip("_").lower()
    return re.sub(r"_+", "_", name)


def rust_field_name(name: str) -> str:
    value = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    value = re.sub(r"[^A-Za-z0-9]+", "_", value).strip("_").lower()
    return re.sub(r"_+", "_", value)


def has_type(schema: Any, type_name: str) -> bool:
    if not isinstance(schema, dict):
        return False
    schema_type = schema.get("type")
    return schema_type == type_name or (
        isinstance(schema_type, list) and type_name in schema_type
    )


def leaf_shapes(schema: Any) -> set[str]:
    if not isinstance(schema, dict):
        return {"unknown"}
    if schema.get("format") == "binary":
        return {"binary"}
    for composition in ("oneOf", "anyOf", "allOf"):
        if composition in schema:
            result: set[str] = set()
            for alternative in schema[composition]:
                result.update(leaf_shapes(alternative))
            return result
    schema_type = schema.get("type")
    if schema_type == "array":
        return {"array"}
    if schema_type == "object" or "properties" in schema or "additionalProperties" in schema:
        return {"object"}
    if isinstance(schema_type, list):
        return {str(item) for item in schema_type if item in SCALAR_TYPES | {"null", "object", "array"}}
    if schema_type in SCALAR_TYPES:
        return {str(schema_type)}
    if schema_type == "null":
        return {"null"}
    return {"unknown"}


def is_supported_scalar_array(spec: dict[str, Any], schema: Any) -> bool:
    if not has_type(schema, "array"):
        return False
    items = resolve(spec, schema.get("items", {}))
    if not isinstance(items, dict) or items.get("format") == "binary":
        return False
    return len(leaf_shapes(items)) == 1 and next(iter(leaf_shapes(items))) in SCALAR_TYPES


def content_type(encoding: dict[str, Any], context: str) -> str | None:
    headers = encoding.get("headers")
    if headers:
        raise ValueError(f"unsupported multipart encoding headers for {context}: {sorted(headers)}")
    if encoding.get("allowReserved") is True:
        raise ValueError(f"unsupported multipart allowReserved=true for {context}")
    value = encoding.get("contentType")
    if value is not None and (not isinstance(value, str) or not value.strip() or "\r" in value or "\n" in value):
        raise ValueError(f"invalid multipart contentType for {context}: {value!r}")
    return value


def encoding_for_array(encoding: dict[str, Any], context: str) -> tuple[bool, str | None]:
    style = encoding.get("style", "form")
    if style != "form":
        raise ValueError(f"unsupported multipart array style for {context}: {style!r}")
    explode = encoding.get("explode", True)
    if not isinstance(explode, bool):
        raise ValueError(f"multipart array explode must be boolean for {context}")
    # OAS 3.1 specifies that an explicitly set style makes contentType,
    # including its implicit value, inapplicable for that property.
    media_type = None if "style" in encoding else content_type(encoding, context)
    if "style" in encoding and encoding.get("headers"):
        raise ValueError(f"unsupported multipart encoding headers for {context}: {sorted(encoding['headers'])}")
    return explode, media_type


def validate_object_encoding(encoding: dict[str, Any], context: str) -> str | None:
    if "style" in encoding or "explode" in encoding:
        raise ValueError(f"unsupported multipart object style/explode override for {context}")
    media_type = content_type(encoding, context)
    if media_type is not None and media_type.lower().split(";", 1)[0].strip() != "application/json":
        raise ValueError(
            f"unsupported multipart object contentType for {context}: {media_type!r}; expected application/json"
        )
    return media_type


def scalar_encoding(encoding: dict[str, Any], context: str) -> str:
    if "style" in encoding or "explode" in encoding:
        raise ValueError(f"unsupported multipart scalar style/explode override for {context}")
    media_type = content_type(encoding, context)
    if media_type is None:
        raise ValueError(f"internal error: scalar encoding has no contentType for {context}")
    return media_type


def validate_scalar_encoding(encoding: dict[str, Any], context: str) -> str | None:
    if "style" in encoding or "explode" in encoding:
        raise ValueError(f"unsupported multipart scalar style/explode override for {context}")
    return content_type(encoding, context)


def normalize_rust(source: str) -> str:
    return re.sub(r"\s+", "", source)


def validate_function_route(function_source: str, method: str, path: str, context: str) -> None:
    methods = re.findall(r"\.request\(reqwest::Method::([A-Z]+),\s*&uri_str\)", function_source)
    if methods != [method.upper()]:
        raise ValueError(
            f"generated function method mismatch for {context}: expected {method.upper()}, found {methods}"
        )
    uri = re.search(r'let uri_str\s*=\s*format!\(\s*"([^\"]+)"', function_source)
    if not uri:
        raise ValueError(f"generated function has no recognized URI mapping for {context}")
    template = uri.group(1)
    if not template.startswith("{}"):
        raise ValueError(f"generated function URI has no configuration base for {context}: {template!r}")
    actual_path = template[2:].replace("{{", "{").replace("}}", "}")
    expected_path = re.sub(r"\{[^{}]+\}", "{}", path)
    actual_path = re.sub(r"\{\}", "{}", actual_path)
    if actual_path != expected_path:
        raise ValueError(
            f"generated function path mismatch for {context}: expected {expected_path!r}, found {actual_path!r}"
        )


def function_region(source: str, function_name: str, context: str) -> tuple[int, int]:
    starts = list(re.finditer(rf"(?m)^pub async fn {re.escape(function_name)}\s*\(", source))
    if len(starts) != 1:
        raise ValueError(
            f"expected one generated function {function_name} for {context}, found {len(starts)}"
        )
    start = starts[0].start()
    end_match = re.search(r"(?m)^}\s*$", source[starts[0].end():])
    if end_match is None:
        raise ValueError(f"could not find end of generated function {function_name} for {context}")
    return start, starts[0].end() + end_match.end()


def selected_json_body_reason(
    spec: dict[str, Any],
    operation: dict[str, Any],
    operation_id: str,
    method: str,
    path: str,
    context: str,
    sources: dict[Path, str],
) -> str | None:
    """Prove a multipart alternative is unused by the generated JSON-only function."""
    raw_request_body = resolve_root(spec, operation.get("requestBody", {}), context)
    content = raw_request_body.get("content", {}) if isinstance(raw_request_body, dict) else {}
    json_media = content.get("application/json") if isinstance(content, dict) else None
    json_schema = json_media.get("schema") if isinstance(json_media, dict) else None
    reference = json_schema.get("$ref") if isinstance(json_schema, dict) else None
    if not isinstance(reference, str) or not reference.startswith("#/components/schemas/"):
        return None
    schema_name = reference.rsplit("/", 1)[-1].replace("~1", "/").replace("~0", "~")

    function_name = operation_name(operation_id)
    matches = [
        (file_path, source)
        for file_path, source in sorted(sources.items())
        if file_path.name.endswith("_api.rs")
        and re.search(rf"(?m)^pub async fn {re.escape(function_name)}\s*\(", source)
    ]
    if len(matches) != 1:
        return None
    file_path, source = matches[0]
    start, finish = function_region(source, function_name, context)
    function = source[start:finish]
    validate_function_route(function, method, path, context)
    multipart_serializers = re.findall(
        r"\.multipart\s*\(|\.form\s*\(|\bmultipart_form\b|\bmultipart_helper::|"
        r"\b(?:reqwest::)?multipart::",
        function,
    )
    if multipart_serializers:
        raise ValueError(
            f"mixed or ambiguous body serialization for {context}: generated function contains "
            f"multipart/form serializers {multipart_serializers} alongside application/json"
        )

    typed_parameters = list(re.finditer(
        rf"\b(?P<name>[A-Za-z_][A-Za-z0-9_]*)\s*:\s*(?:models::)?{re.escape(schema_name)}\b",
        function,
    ))
    if len(typed_parameters) != 1:
        return None
    parameter = typed_parameters[0].group("name")
    body_alias = f"p_body_{parameter}"
    if not re.search(rf"\blet\s+{re.escape(body_alias)}\s*=\s*{re.escape(parameter)}\s*;", function):
        return None
    if not re.search(rf"\.json\s*\(\s*&{re.escape(body_alias)}\s*\)", function):
        return None
    return (
        f"multipart alternative excluded: generated {file_path.name}::{function_name} maps "
        f"{method.upper()} {path} to application/json schema {reference} and serializes that exact "
        f"parameter with req_builder.json(&{body_alias}); the generated function has no multipart serializer"
    )


def block_end(source: str, opening: int, context: str) -> int:
    depth = 0
    in_string = False
    escaped = False
    line_comment = False
    block_comment = False
    i = opening
    while i < len(source):
        char = source[i]
        nxt = source[i + 1] if i + 1 < len(source) else ""
        if line_comment:
            if char == "\n":
                line_comment = False
        elif block_comment:
            if char == "*" and nxt == "/":
                block_comment = False
                i += 1
        elif in_string:
            if escaped:
                escaped = False
            elif char == "\\":
                escaped = True
            elif char == '"':
                in_string = False
        elif char == "/" and nxt == "/":
            line_comment = True
            i += 1
        elif char == "/" and nxt == "*":
            block_comment = True
            i += 1
        elif char == '"':
            in_string = True
        elif char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return i + 1
        i += 1
    raise ValueError(f"unterminated generated field block for {context}")


def replace_optional_field_block(
    function_source: str, field: str, replacement: str, context: str
) -> str:
    variable = rust_field_name(field)
    marker = f"if let Some(param_value) = p_form_{variable} {{"
    positions = [m.start() for m in re.finditer(re.escape(marker), function_source)]
    if len(positions) != 1:
        raise ValueError(f"expected one optional generated serializer for {context}, found {len(positions)}")
    opening = function_source.find("{", positions[0] + len(marker) - 1)
    end = block_end(function_source, opening, context)
    line_start = function_source.rfind("\n", 0, positions[0]) + 1
    indent = function_source[line_start:positions[0]]
    if indent.strip():
        raise ValueError(f"generated serializer for {context} is not on its own line")
    old_block = function_source[line_start:end]
    if not re.search(
        rf"\.text\(\s*{re.escape(json.dumps(field))}\s*,|"
        rf"multipart_helper::add_schema_value_to_form\(\s*multipart_form,\s*{re.escape(json.dumps(field))}\s*,",
        old_block,
    ):
        raise ValueError(f"generated serializer wire field does not match {context}")
    replacement = indent + replacement.replace("\n", "\n" + indent)
    if "add_schema_value_to_form" in old_block:
        if normalize_rust(old_block) != normalize_rust(replacement):
            raise ValueError(f"already-fixed multipart serializer does not match expected semantics for {context}")
        return function_source
    return function_source[:line_start] + replacement + function_source[end:]


def replace_required_field(
    function_source: str, field: str, replacement: str, context: str
) -> str:
    variable = rust_field_name(field)
    line = re.compile(
        rf"(?m)^(?P<indent>\s*)multipart_form = multipart_form\.text\(\"{re.escape(field)}\", p_form_{re.escape(variable)}\.to_string\(\)\);\s*$"
    )
    matches = list(line.finditer(function_source))
    if len(matches) != 1:
        raise ValueError(f"expected one required generated serializer for {context}, found {len(matches)}")
    match = matches[0]
    indented = "\n".join(match.group("indent") + line for line in replacement.splitlines())
    if "add_schema_value_to_form" in function_source:
        raise ValueError(f"required multipart serializer has an ambiguous existing helper for {context}")
    return function_source[:match.start()] + indented + function_source[match.end():]


def helper_call(field: str, value: str, media_type: str | None) -> str:
    content_type_value = "None" if media_type is None else f"Some({json.dumps(media_type)})"
    return (
        "multipart_form = multipart_helper::add_schema_value_to_form(\n"
        "    multipart_form,\n"
        f"    {json.dumps(field)},\n"
        f"    {value},\n"
        f"    {content_type_value},\n"
        ")?;"
    )


def ensure_helper_import(source: str, file_path: Path) -> str:
    if re.search(r"(?m)^use super::\{[^}]*\bmultipart_helper\b[^}]*\};", source):
        return source
    match = re.search(r"(?m)^use super::\{([^}]+)\};", source)
    if not match:
        raise ValueError(f"cannot add multipart_helper import in {file_path}: no grouped super import")
    imports = match.group(1).strip()
    updated = imports + ("" if imports.endswith(",") else ",") + " multipart_helper"
    return source[:match.start(1)] + updated + source[match.end(1):]


def patch_function(
    apis_dir: Path,
    sources: dict[Path, str],
    operation_id: str,
    field: str,
    patch_kind: str,
    replacement: str,
    method: str,
    path: str,
    context: str,
) -> Path:
    function_name = operation_name(operation_id)
    matches = []
    for file_path in sorted(sources):
        if not file_path.name.endswith("_api.rs"):
            continue
        source = sources[file_path]
        if re.search(rf"(?m)^pub async fn {re.escape(function_name)}\s*\(", source):
            matches.append((file_path, source))
    if len(matches) != 1:
        raise ValueError(f"expected one API file for {context}, found {len(matches)}")
    file_path, source = matches[0]
    start, end = function_region(source, function_name, context)
    function_source = source[start:end]
    validate_function_route(function_source, method, path, context)
    field_call = re.compile(
        rf"multipart_helper::add_schema_value_to_form\(\s*multipart_form,\s*{re.escape(json.dumps(field))}\s*,"
    )
    if field_call.search(function_source) and normalize_rust(replacement) not in normalize_rust(function_source):
        raise ValueError(f"already-fixed multipart serializer does not match expected semantics for {context}")
    if patch_kind == "required":
        if field_call.search(function_source):
            if normalize_rust(replacement) not in normalize_rust(function_source):
                raise ValueError(f"already-fixed multipart serializer does not match expected semantics for {context}")
        else:
            function_source = replace_required_field(function_source, field, replacement, context)
    else:
        function_source = replace_optional_field_block(function_source, field, replacement, context)
    updated_source = source[:start] + function_source + source[end:]
    updated_source = ensure_helper_import(updated_source, file_path)
    sources[file_path] = updated_source
    return file_path


def find_binary_shape(schema: Any) -> bool:
    if not isinstance(schema, dict):
        return False
    if schema.get("format") == "binary":
        return True
    for key in ("items", "oneOf", "anyOf", "allOf", "properties"):
        value = schema.get(key)
        if isinstance(value, dict) and find_binary_shape(value):
            return True
        if isinstance(value, list) and any(find_binary_shape(item) for item in value):
            return True
    return False


def fix_spec(spec: dict[str, Any], apis_dir: Path) -> dict[str, Any]:
    changes: list[dict[str, str]] = []
    skipped_binary: list[dict[str, str]] = []
    skipped_unsupported_roots: list[dict[str, str]] = []
    original_sources = {
        file_path: file_path.read_text(encoding="utf-8")
        for file_path in sorted(apis_dir.glob("*_api.rs"))
    }
    planned_sources = dict(original_sources)
    for path, raw_path_item in spec.get("paths", {}).items():
        try:
            path_item = resolve_root(spec, raw_path_item, f"Path Item {path}")
        except ValueError as exc:
            raise ValueError(f"cannot resolve Path Item {path}: {exc}") from exc
        if not isinstance(path_item, dict):
            raise ValueError(f"Path Item {path} is not an object")
        for method, operation in path_item.items():
            if method.lower() not in HTTP_METHODS or not isinstance(operation, dict):
                continue
            operation_context = f"{method.upper()} {path}"
            operation = resolve_root(spec, operation, operation_context)
            operation_id = operation.get("operationId")
            request_body = resolve(spec, operation.get("requestBody", {}))
            if not isinstance(request_body, dict):
                continue
            content = request_body.get("content", {})
            if not isinstance(content, dict):
                raise ValueError(f"requestBody content is not an object for {operation_context}")
            if "multipart/form-data" not in content:
                continue
            multipart = content["multipart/form-data"]
            if not isinstance(multipart, dict):
                raise ValueError(f"multipart media type is not an object for {operation_context}")
            if not operation_id:
                raise ValueError(f"multipart operation has no operationId for {operation_context}")
            schema = resolve(spec, multipart.get("schema", {}))
            if not isinstance(schema, dict):
                raise ValueError(f"multipart schema is not an object for {operation_context} ({operation_id})")
            root_compositions = [key for key in ("allOf", "oneOf", "anyOf") if key in schema]
            if root_compositions:
                exclusion_reason = selected_json_body_reason(
                    spec,
                    operation,
                    operation_id,
                    method,
                    path,
                    f"{operation_context} ({operation_id})",
                    planned_sources,
                )
                if exclusion_reason:
                    skipped_unsupported_roots.append({
                        "operation": operation_context,
                        "operation_id": operation_id,
                        "composition": ",".join(root_compositions),
                        "reason": exclusion_reason,
                    })
                    continue
                raise ValueError(
                    f"unsupported multipart root composition for {operation_context} ({operation_id}): "
                    f"{root_compositions}; direct properties are required"
                )
            properties = schema.get("properties")
            if not isinstance(properties, dict):
                raise ValueError(
                    f"unsupported multipart root shape for {operation_context} ({operation_id}): "
                    "expected a direct object properties map"
                )
            encodings = multipart.get("encoding", {})
            if not isinstance(encodings, dict):
                raise ValueError(f"multipart encoding is not an object for {operation_context} ({operation_id})")
            unknown_encoding_fields = sorted(set(encodings) - set(properties))
            if unknown_encoding_fields:
                raise ValueError(
                    f"multipart encoding names unknown properties for {operation_context} ({operation_id}): "
                    f"{unknown_encoding_fields}"
                )

            rust_names: dict[str, str] = {}
            for field in properties:
                normalized = rust_field_name(field)
                previous = rust_names.get(normalized)
                if previous is not None:
                    raise ValueError(
                        f"normalized Rust field collision for {operation_context} ({operation_id}): "
                        f"{previous!r} and {field!r} both map to {normalized!r}"
                    )
                rust_names[normalized] = field

            for field, raw_property in properties.items():
                context = f"{operation_context} ({operation_id}).{field}"
                property_schema = resolve(spec, raw_property)
                shape = leaf_shapes(property_schema)
                encoding = encodings.get(field, {})
                if not isinstance(encoding, dict):
                    raise ValueError(f"multipart encoding for {context} is not an object")
                allowed_encoding_keys = {"contentType", "headers", "style", "explode", "allowReserved"}
                unknown_encoding_keys = sorted(set(encoding) - allowed_encoding_keys)
                if unknown_encoding_keys:
                    raise ValueError(f"unsupported multipart encoding keys for {context}: {unknown_encoding_keys}")

                if find_binary_shape(property_schema):
                    if encoding:
                        raise ValueError(
                            f"unsupported multipart binary encoding override for {context}: "
                            f"{sorted(encoding)}"
                        )
                    skipped_binary.append({
                        "operation": f"{method.upper()} {path}",
                        "operation_id": operation_id,
                        "field": field,
                        "reason": "binary or file-union serialization remains owned by patch_multipart.sh",
                    })
                    continue

                is_array = has_type(property_schema, "array")
                is_object_or_object_union = "object" in shape
                explicit_content_type = "contentType" in encoding

                if is_array:
                    items = resolve(spec, property_schema.get("items", {}))
                    item_shapes = leaf_shapes(items)
                    if not is_supported_scalar_array(spec, property_schema) and item_shapes != {"object"}:
                        raise ValueError(f"unsupported multipart array item shape for {context}: {sorted(item_shapes)}")
                    if "style" not in encoding and "explode" not in encoding:
                        explode, media_type = True, content_type(encoding, context)
                    else:
                        explode, media_type = encoding_for_array(encoding, context)
                    if item_shapes == {"object"} and not explode:
                        raise ValueError(f"non-exploded multipart arrays of objects are unsupported for {context}")
                    if explode:
                        value = "serde_json::to_value(item)?"
                        code = (
                            f"if let Some(param_value) = p_form_{rust_field_name(field)} {{\n"
                            "    for item in param_value {\n"
                            f"        {helper_call(field, value, media_type).replace(chr(10), chr(10) + '        ')}\n"
                            "    }\n"
                            "}"
                        )
                        # The helper call references the loop's item and appends one part per value.
                    else:
                        joined = (
                            "serde_json::Value::String(param_value.into_iter()"
                            ".map(|item| item.to_string()).collect::<Vec<String>>().join(\",\"))"
                        )
                        code = (
                            f"if let Some(param_value) = p_form_{rust_field_name(field)} {{\n"
                            "    if !param_value.is_empty() {\n"
                            f"        {helper_call(field, joined, media_type).replace(chr(10), chr(10) + '        ')}\n"
                            "    }\n"
                            "}"
                        )
                    file_path = patch_function(
                        apis_dir, planned_sources, operation_id, field, "optional", code, method, path, context
                    )
                    changes.append({"operation": context, "file": file_path.name, "rule": f"array form explode={explode}"})
                    continue

                if is_object_or_object_union:
                    if not shape <= {"object", "string", "integer", "number", "boolean", "null"}:
                        raise ValueError(f"unsupported multipart object union shape for {context}: {sorted(shape)}")
                    media_type = validate_object_encoding(encoding, context)
                    code = (
                        f"if let Some(param_value) = p_form_{rust_field_name(field)} {{\n"
                        f"    {helper_call(field, 'serde_json::to_value(param_value)?', media_type).replace(chr(10), chr(10) + '    ')}\n"
                        "}"
                    )
                    file_path = patch_function(
                        apis_dir, planned_sources, operation_id, field, "optional", code, method, path, context
                    )
                    changes.append({"operation": context, "file": file_path.name, "rule": "JSON object or value-shape multipart part"})
                    continue

                if shape <= SCALAR_TYPES | {"null"}:
                    media_type = validate_scalar_encoding(encoding, context)
                else:
                    raise ValueError(f"unsupported multipart property shape for {context}: {sorted(shape)}")

                if explicit_content_type:
                    assert media_type is not None
                    # Required scalar parts use the generator's direct `.text` form;
                    # optional scalar fields use the standard Option block.
                    function_name = operation_name(operation_id)
                    api_matches = [
                        f for f in planned_sources
                        if f.name.endswith("_api.rs")
                        and re.search(rf"(?m)^pub async fn {re.escape(function_name)}\s*\(", planned_sources[f])
                    ]
                    if len(api_matches) != 1:
                        raise ValueError(f"expected one API file for {context}, found {len(api_matches)}")
                    function_source = planned_sources[api_matches[0]]
                    begin, finish = function_region(function_source, function_name, context)
                    region = function_source[begin:finish]
                    optional_marker = f"if let Some(param_value) = p_form_{rust_field_name(field)} {{"
                    if optional_marker in region:
                        code = (
                            f"if let Some(param_value) = p_form_{rust_field_name(field)} {{\n"
                            f"    {helper_call(field, 'serde_json::Value::String(param_value.to_string())', media_type).replace(chr(10), chr(10) + '    ')}\n"
                            "}"
                        )
                        file_path = patch_function(
                            apis_dir, planned_sources, operation_id, field, "optional", code, method, path, context
                        )
                    else:
                        value = f"serde_json::Value::String(p_form_{rust_field_name(field)}.to_string())"
                        file_path = patch_function(
                            apis_dir,
                            planned_sources,
                            operation_id,
                            field,
                            "required",
                            helper_call(field, value, media_type),
                            method,
                            path,
                            context,
                        )
                    changes.append({"operation": context, "file": file_path.name, "rule": f"explicit contentType={media_type}"})

    for file_path, updated_source in planned_sources.items():
        if updated_source != original_sources[file_path]:
            file_path.write_text(updated_source, encoding="utf-8")
    return {
        "status": "passed",
        "changes": changes,
        "skipped_binary_fields": skipped_binary,
        "skipped_unsupported_roots": skipped_unsupported_roots,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("spec", type=Path)
    parser.add_argument("apis_dir", type=Path)
    parser.add_argument("--report", type=Path)
    args = parser.parse_args()
    try:
        spec = yaml.safe_load(args.spec.read_text(encoding="utf-8"))
        result = fix_spec(spec, args.apis_dir)
        if args.report:
            args.report.parent.mkdir(parents=True, exist_ok=True)
            args.report.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
        print(json.dumps(result, indent=2))
        return 0
    except (OSError, ValueError, yaml.YAMLError) as exc:
        print(f"multipart serialization fix failed: {exc}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
