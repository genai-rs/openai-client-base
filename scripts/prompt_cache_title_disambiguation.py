"""Disambiguate the Decisions request's inline prompt-cache model for codegen."""

from __future__ import annotations

from typing import Any


INLINE_TITLE = "DecisionRequestPromptCacheOptions"
UPSTREAM_TITLE = "Prompt cache options"


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def _is_nullable_object_type(value: Any) -> bool:
    return (
        isinstance(value, list)
        and len(value) == 2
        and all(isinstance(item, str) for item in value)
        and set(value) == {"object", "null"}
    )


def _is_nullable_string_type(value: Any) -> bool:
    return (
        isinstance(value, list)
        and len(value) == 2
        and all(isinstance(item, str) for item in value)
        and set(value) == {"string", "null"}
    )


def _validate_response_schema(response: dict[str, Any]) -> None:
    _require(
        response.get("type") == "object",
        "PromptCacheOptions response schema is no longer an object",
    )
    _require(
        response.get("title") == UPSTREAM_TITLE,
        "PromptCacheOptions upstream title changed; review generator naming before patching",
    )
    required = response.get("required")
    _require(
        isinstance(required, list)
        and all(isinstance(field, str) for field in required)
        and set(required) == {"ttl", "mode"},
        "PromptCacheOptions response no longer requires exactly ttl and mode",
    )
    properties = response.get("properties")
    _require(
        isinstance(properties, dict),
        "PromptCacheOptions response properties are missing or malformed",
    )
    for field, ref_name in (
        ("ttl", "PromptCacheTTLEnum"),
        ("mode", "PromptCacheModeEnum"),
    ):
        prop = properties.get(field)
        _require(
            isinstance(prop, dict)
            and prop.get("$ref") == f"#/components/schemas/{ref_name}"
            and "nullable" not in prop
            and "default" not in prop,
            f"PromptCacheOptions.{field} no longer has the expected required non-null reference",
        )


def _validate_inline_schema(inline: dict[str, Any]) -> None:
    _require(
        _is_nullable_object_type(inline.get("type")),
        "DecisionRequest.prompt_cache_options must remain a nullable inline object",
    )
    required = inline.get("required", [])
    _require(
        isinstance(required, list) and not required,
        "DecisionRequest.prompt_cache_options must keep ttl and mode optional",
    )
    properties = inline.get("properties")
    _require(
        isinstance(properties, dict),
        "DecisionRequest.prompt_cache_options properties are missing or malformed",
    )

    ttl = properties.get("ttl")
    _require(
        isinstance(ttl, dict)
        and ttl.get("type") == "string"
        and ttl.get("enum") == ["30m"]
        and ttl.get("default", "30m") == "30m"
        and "nullable" not in ttl,
        "DecisionRequest.prompt_cache_options.ttl no longer matches the optional non-null upstream field",
    )
    mode = properties.get("mode")
    _require(
        isinstance(mode, dict)
        and _is_nullable_string_type(mode.get("type"))
        and isinstance(mode.get("enum"), list)
        and all(isinstance(value, str) or value is None for value in mode["enum"])
        and set(mode["enum"]) == {"implicit", "explicit", None},
        "DecisionRequest.prompt_cache_options.mode no longer matches the optional nullable upstream field",
    )


def disambiguate_decision_request_prompt_cache_options(
    schemas: dict[str, Any],
) -> bool:
    """Give the inline request schema a unique generator-only title.

    OpenAPI Generator maps the inline request schema and the named response
    schema to the same Rust model path when their upstream titles match. This
    function changes only the inline title. Unrelated specs that do not contain
    the affected inline site pass through unchanged.
    """
    decision_request = schemas.get("DecisionRequest")
    if decision_request is None:
        return False
    _require(
        isinstance(decision_request, dict),
        "DecisionRequest schema is malformed while checking prompt-cache model naming",
    )
    properties = decision_request.get("properties")
    if properties is None:
        return False
    _require(
        isinstance(properties, dict),
        "DecisionRequest.properties is malformed while checking prompt-cache model naming",
    )
    if "prompt_cache_options" not in properties:
        return False

    inline = properties["prompt_cache_options"]
    _require(
        isinstance(inline, dict) and "$ref" not in inline,
        "DecisionRequest.prompt_cache_options is no longer the expected inline schema",
    )
    response = schemas.get("PromptCacheOptions")
    _require(
        isinstance(response, dict),
        "DecisionRequest.prompt_cache_options exists but named PromptCacheOptions response schema is missing",
    )
    _validate_response_schema(response)
    _validate_inline_schema(inline)

    # Check the generated component name before the idempotent-title return so
    # an upstream component cannot be silently shadowed on repeated runs.
    _require(
        INLINE_TITLE not in schemas,
        f"generator disambiguation title collides with component schema {INLINE_TITLE}",
    )

    title = inline.get("title")
    if title == INLINE_TITLE:
        return False
    _require(
        title == UPSTREAM_TITLE,
        "DecisionRequest inline prompt-cache title changed; review generator naming before patching",
    )
    inline["title"] = INLINE_TITLE
    return True
