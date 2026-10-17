import copy
import hashlib
import json
import os
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from api_change_report import compare, inventory, markdown, snapshot

FIXTURES = Path(__file__).parent / "fixtures" / "api_change_report"
SCRIPT = Path(__file__).resolve().parents[1] / "api_change_report.py"


class APIChangeReportTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for folder in ("apis", "models"):
            path = self.root / "src" / folder
            path.mkdir(parents=True)
            (path / "mod.rs").write_text("// fixture\n")

    def capture(self, name):
        return snapshot(FIXTURES / f"{name}.json", self.root,
                        "https://example.test/openapi.json", "7.15.0", "a" * 40)

    def spec(self):
        return json.loads((FIXTURES / "before.json").read_text())

    def test_breaking_fixture_flags_removal_requiredness_and_enum_wire_value(self):
        report = compare(self.capture("before"), self.capture("breaking"))
        self.assertTrue(report["review_required"])
        self.assertTrue(report["removals_flagged"])
        self.assertEqual(report["changes"]["operations"]["removed"], ["POST /widgets"])
        self.assertEqual(report["changes"]["models"]["removed"], ["Legacy"])
        self.assertEqual(report["changes"]["required"], [{
            "location": "#/components/schemas/Widget", "removed": ["status"], "added": ["metadata"]}])
        self.assertEqual(report["changes"]["enums"], [{
            "location": "#/components/schemas/Widget/properties/status", "removed": ['"pending"'], "added": []}])

    def test_reordered_equivalent_input_has_no_changes_but_keeps_raw_hashes(self):
        before, after = self.capture("before"), self.capture("reordered")
        self.assertNotEqual(before["provenance"]["input_sha256"], after["provenance"]["input_sha256"])
        report = compare(before, after)
        self.assertFalse(report["review_required"])
        self.assertFalse(report["removals_flagged"])
        self.assertIn("No covered changes detected", markdown(report))

    def test_additive_fixture_does_not_claim_removals_or_compatibility(self):
        report = compare(self.capture("before"), self.capture("additive"))
        self.assertFalse(report["removals_flagged"])
        self.assertTrue(report["review_required"])
        self.assertEqual(report["changes"]["models"]["added"], ["NewModel"])
        self.assertEqual(report["changes"]["operations"]["added"], ["GET /widgets/{id}"])
        self.assertIn("not complete Rust semver analysis", markdown(report))

    def test_nested_inline_schema_and_operation_rename(self):
        before = self.capture("before")
        after = copy.deepcopy(before)
        spec = self.spec()
        spec["paths"]["/widgets"]["get"]["operationId"] = "getWidgets"
        spec["paths"]["/widgets"]["post"]["requestBody"] = {"content": {"application/json": {
            "schema": {"type": "object", "required": ["mode"], "properties": {
                "mode": {"type": "string", "enum": ["strict"]}}}}}}
        spec["components"]["schemas"]["Widget"]["properties"]["metadata"]["required"] = ["tag"]
        after["surface"] = inventory(spec)
        report = compare(before, after)
        self.assertEqual(report["changes"]["operation_ids"], [{
            "operation": "GET /widgets", "before": "listWidgets", "after": "getWidgets"}])
        self.assertEqual(len(report["changes"]["required"]), 2)
        self.assertEqual(len(report["changes"]["enums"]), 1)

    def test_union_reordering_and_wire_types(self):
        spec = self.spec()
        model = {"anyOf": [{"type": "string", "enum": ["0", "false"]},
                           {"enum": [False, 0, None]}]}
        spec["components"]["schemas"]["Union"] = model
        before = inventory(spec)
        model["anyOf"].reverse()
        for branch in model["anyOf"]:
            branch["enum"].reverse()
        self.assertEqual(before, inventory(spec))
        values = {v for values in before["enums"].values() for v in values}
        self.assertTrue({'"0"', '"false"', 'false', '0', 'null'} <= values)

    def test_examples_and_properties_named_like_schema_keywords(self):
        spec = self.spec()
        spec["components"]["schemas"]["Keywords"] = {
            "type": "object", "properties": {"enum": {"type": "string"},
            "required": {"type": "boolean"}, "schema": {"type": "string"}},
            "example": {"enum": ["not-a-wire-value"], "$ref": "not-a-reference", "required": ["fake"]}}
        surface = inventory(spec)
        self.assertFalse(any("Keywords" in name for name in surface["enums"]))

    def test_unresolved_reference_is_error_without_backfill(self):
        spec = self.spec()
        del spec["components"]["schemas"]["Widget"]
        original = copy.deepcopy(spec)
        with self.assertRaisesRegex(ValueError, "unresolved reference"):
            inventory(spec)
        self.assertEqual(spec, original)
        spec["paths"]["/external"] = {"$ref": "https://example.test/external"}
        with self.assertRaisesRegex(ValueError, "reference"):
            inventory(spec)

    def test_local_path_reference_and_empty_surface(self):
        spec = self.spec()
        spec["components"]["pathItems"] = {"Widgets": spec["paths"].pop("/widgets")}
        spec["paths"]["/widgets"] = {"$ref": "#/components/pathItems/Widgets"}
        self.assertIn("POST /widgets", inventory(spec)["operations"])
        self.assertEqual(inventory({"openapi": "3.1.0", "paths": {}})["operations"], {})

    def test_generated_file_removal_addition_and_content_change(self):
        model = self.root / "src/models/widget.rs"
        model.write_text("pub struct Widget;\n")
        before = self.capture("before")
        model.unlink()
        (model.parent / "new.rs").write_text("pub struct New;\n")
        (self.root / "src/apis/mod.rs").write_text("// changed\n")
        report = compare(before, self.capture("before"))
        self.assertEqual(report["changes"]["generated_files"], {
            "removed": ["src/models/widget.rs"], "added": ["src/models/new.rs"], "changed": ["src/apis/mod.rs"]})
        self.assertTrue(report["removals_flagged"])

    def test_yaml_keeps_enum_strings_and_booleans_distinct(self):
        path = self.root / "spec.yaml"
        path.write_text('openapi: 3.1.0\npaths: {}\ncomponents:\n  schemas:\n    Mode:\n      enum: [on, off, 2026-10-17, true, false, "false"]\n')
        snap = snapshot(path, self.root, "https://example.test/spec", "7.15.0", "a" * 40)
        self.assertEqual(set(snap["surface"]["enums"]["#/components/schemas/Mode"]),
                         {'"on"', '"off"', '"2026-10-17"', 'true', 'false', '"false"'})

    def test_cli_is_deterministic_read_only_and_needs_no_rustdoc_or_subprocess_tools(self):
        source = self.root / "spec.json"
        source.write_bytes((FIXTURES / "before.json").read_bytes())
        source_files = [source, *self.root.rglob("*.rs")]
        original = {p: p.read_bytes() for p in source_files}
        # Only the Python interpreter is available. No rustdoc, cargo, git or network CLI.
        env = {**os.environ, "PATH": str(self.root / "absent"), "PYTHONDONTWRITEBYTECODE": "1"}
        command = [sys.executable, str(SCRIPT), "snapshot", "--spec", str(source),
                   "--generated-root", str(self.root), "--source-url", "https://example.test/spec",
                   "--generator-version", "7.15.0", "--revision", "a" * 40, "--output", str(self.root / "before.json")]
        subprocess.run(command, env=env, check=True, capture_output=True)
        captured = (self.root / "before.json").read_bytes()
        subprocess.run(command, env=env, check=True, capture_output=True)
        self.assertEqual(captured, (self.root / "before.json").read_bytes())
        self.assertEqual(json.loads(captured)["provenance"]["input_sha256"], hashlib.sha256(original[source]).hexdigest())
        compare_command = [sys.executable, str(SCRIPT), "compare", "--before", str(self.root / "before.json"),
                           "--after", str(self.root / "before.json"), "--json-output", str(self.root / "report.json"),
                           "--markdown-output", str(self.root / "report.md")]
        subprocess.run(compare_command, env=env, check=True, capture_output=True)
        outputs = [(self.root / name).read_bytes() for name in ("report.json", "report.md")]
        subprocess.run(compare_command, env=env, check=True, capture_output=True)
        self.assertEqual(outputs, [(self.root / name).read_bytes() for name in ("report.json", "report.md")])
        self.assertEqual(original, {p: p.read_bytes() for p in source_files})
        overwrite = subprocess.run(command[:-1] + [str(source)], env=env, capture_output=True)
        self.assertEqual(overwrite.returncode, 2)
        self.assertEqual(source.read_bytes(), original[source])

    def test_missing_generated_outputs_and_invalid_snapshots_fail(self):
        (self.root / "src/models/mod.rs").unlink()
        with self.assertRaisesRegex(ValueError, "no generated Rust files"):
            self.capture("before")
        with self.assertRaisesRegex(ValueError, "format version"):
            compare({"format_version": 999}, {})

    def test_duplicate_yaml_keys_and_empty_provenance_are_rejected(self):
        path = self.root / "duplicate.yaml"
        path.write_text("openapi: 3.1.0\npaths: {}\npaths: {}\n")
        with self.assertRaisesRegex(ValueError, "duplicate OpenAPI key"):
            snapshot(path, self.root, "https://example.test/spec", "7.15.0", "a" * 40)
        with self.assertRaisesRegex(ValueError, "required"):
            snapshot(FIXTURES / "before.json", self.root, "", "7.15.0", "a" * 40)

    def test_report_escapes_upstream_markdown(self):
        before, after = self.capture("before"), self.capture("before")
        before["surface"]["models"].append("<script>|\n[link](https://example.test)")
        rendered = markdown(compare(before, after))
        self.assertIn("&lt;script&gt;&#124;&#10;", rendered)
        self.assertNotIn("<script>", rendered)


if __name__ == "__main__":
    unittest.main()
