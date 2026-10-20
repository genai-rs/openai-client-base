import copy
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from prompt_cache_title_disambiguation import (
    INLINE_TITLE,
    UPSTREAM_TITLE,
    disambiguate_decision_request_prompt_cache_options,
)


SCRIPT = Path(__file__).resolve().parents[1] / "patch_spec_rust_compat.py"


class PromptCacheTitleDisambiguationTests(unittest.TestCase):
    def setUp(self):
        self.schemas = {
            "PromptCacheOptions": {
                "type": "object",
                "title": UPSTREAM_TITLE,
                "required": ["ttl", "mode"],
                "properties": {
                    "ttl": {"$ref": "#/components/schemas/PromptCacheTTLEnum"},
                    "mode": {"$ref": "#/components/schemas/PromptCacheModeEnum"},
                    "comparison_response_id": {
                        "anyOf": [{"type": "string"}, {"type": "null"}]
                    },
                },
            },
            "PromptCacheOptionsParam": {
                "type": "object",
                "title": UPSTREAM_TITLE,
                "required": [],
                "properties": {
                    "ttl": {"$ref": "#/components/schemas/PromptCacheTTLEnum"},
                    "mode": {"$ref": "#/components/schemas/PromptCacheModeEnum"},
                },
            },
            "DecisionRequest": {
                "type": "object",
                "required": ["model"],
                "properties": {
                    "model": {"type": "string"},
                    "prompt_cache_options": {
                        "type": ["object", "null"],
                        "title": UPSTREAM_TITLE,
                        "properties": {
                            "ttl": {
                                "type": "string",
                                "enum": ["30m"],
                                "default": "30m",
                            },
                            "mode": {
                                "type": ["string", "null"],
                                "enum": ["implicit", "explicit", None],
                            },
                        },
                    },
                },
            },
        }

    def assert_unchanged_error(self, schemas, expected_message):
        before = copy.deepcopy(schemas)
        with self.assertRaisesRegex(ValueError, expected_message):
            disambiguate_decision_request_prompt_cache_options(schemas)
        self.assertEqual(schemas, before)

    def test_changes_only_inline_title_and_is_idempotent(self):
        before = copy.deepcopy(self.schemas)
        changed = disambiguate_decision_request_prompt_cache_options(self.schemas)
        self.assertTrue(changed)
        expected = copy.deepcopy(before)
        expected["DecisionRequest"]["properties"]["prompt_cache_options"][
            "title"
        ] = INLINE_TITLE
        self.assertEqual(self.schemas, expected)
        self.assertFalse(
            disambiguate_decision_request_prompt_cache_options(self.schemas)
        )

    def test_unrelated_widget_spec_is_a_noop(self):
        schemas = {"Widget": {"type": "object", "properties": {"id": {"type": "string"}}}}
        before = copy.deepcopy(schemas)
        self.assertFalse(disambiguate_decision_request_prompt_cache_options(schemas))
        self.assertEqual(schemas, before)

    def test_absent_inline_site_is_a_noop(self):
        schemas = {
            "PromptCacheOptions": copy.deepcopy(self.schemas["PromptCacheOptions"]),
            "DecisionRequest": {
                "type": "object",
                "properties": {"model": {"type": "string"}},
            },
        }
        before = copy.deepcopy(schemas)
        self.assertFalse(disambiguate_decision_request_prompt_cache_options(schemas))
        self.assertEqual(schemas, before)

    def test_generic_patch_script_accepts_widget_only_openapi(self):
        spec = {
            "openapi": "3.1.0",
            "info": {"title": "Widget fixture", "version": "1"},
            "paths": {},
            "components": {"schemas": self._widget_schemas()},
        }
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "target" / "reports").mkdir(parents=True)
            input_path = root / "widget.json"
            output_path = root / "widget.out.yaml"
            input_path.write_text(json.dumps(spec))
            result = subprocess.run(
                [sys.executable, str(SCRIPT), str(input_path), str(output_path)],
                cwd=root,
                capture_output=True,
                text=True,
                check=False,
            )
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            self.assertTrue(output_path.is_file())

    def test_missing_named_response_fails_closed_without_mutating(self):
        schemas = {"DecisionRequest": copy.deepcopy(self.schemas["DecisionRequest"])}
        self.assert_unchanged_error(schemas, "named PromptCacheOptions response schema is missing")

    def test_malformed_response_requiredness_fails_closed_without_mutating(self):
        schemas = copy.deepcopy(self.schemas)
        schemas["PromptCacheOptions"]["required"] = ["ttl"]
        self.assert_unchanged_error(schemas, "no longer requires exactly ttl and mode")

    def test_malformed_response_refs_fail_closed_without_mutating(self):
        schemas = copy.deepcopy(self.schemas)
        schemas["PromptCacheOptions"]["properties"]["ttl"]["$ref"] = "#/components/schemas/Other"
        self.assert_unchanged_error(schemas, "required non-null reference")

    def test_nonnullable_inline_object_fails_closed_without_mutating(self):
        schemas = copy.deepcopy(self.schemas)
        schemas["DecisionRequest"]["properties"]["prompt_cache_options"]["type"] = [
            "object"
        ]
        self.assert_unchanged_error(schemas, "must remain a nullable inline object")

    def test_malformed_inline_mode_fails_closed_without_mutating(self):
        schemas = copy.deepcopy(self.schemas)
        schemas["DecisionRequest"]["properties"]["prompt_cache_options"]["properties"][
            "mode"
        ]["type"] = "string"
        self.assert_unchanged_error(schemas, "mode no longer matches")

    def test_component_collision_fails_on_first_call_without_mutating(self):
        schemas = copy.deepcopy(self.schemas)
        schemas[INLINE_TITLE] = {"type": "object"}
        self.assert_unchanged_error(schemas, "collides with component schema")

    def test_component_collision_fails_on_repeated_call_without_mutating(self):
        schemas = copy.deepcopy(self.schemas)
        self.assertTrue(disambiguate_decision_request_prompt_cache_options(schemas))
        schemas[INLINE_TITLE] = {"type": "object"}
        self.assert_unchanged_error(schemas, "collides with component schema")

    @staticmethod
    def _widget_schemas():
        return {"Widget": {"type": "object", "properties": {"id": {"type": "string"}}}}


if __name__ == "__main__":
    unittest.main()
