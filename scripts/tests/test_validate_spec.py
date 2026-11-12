import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from validate_spec import validate

SCRIPT = Path(__file__).resolve().parents[1] / "validate_spec.py"


class ValidateSpecTests(unittest.TestCase):
    def test_rejects_http_error_document(self):
        self.assertIn("missing or unsupported OpenAPI version", validate({"error": "not found"}))

    def test_rejects_missing_schema_without_replacing_it(self):
        spec = {
            "openapi": "3.1.0",
            "paths": {"/test": {"get": {"responses": {"200": {"$ref": "#/components/schemas/Missing"}}}}},
            "components": {"schemas": {"Present": {"type": "object"}}},
        }
        self.assertEqual(
            ["#/paths//test/get/responses/200: unresolved $ref #/components/schemas/Missing"],
            validate(spec),
        )
        self.assertEqual({"Present": {"type": "object"}}, spec["components"]["schemas"])

    def test_accepts_escaped_local_pointer(self):
        spec = {
            "openapi": "3.1.0",
            "paths": {"/test": {"get": {"$ref": "#/components/schemas/With~1Slash"}}},
            "components": {"schemas": {"With/Slash": {"type": "object"}}},
        }
        self.assertEqual([], validate(spec))

    def test_cli_reads_json_and_rejects_non_json_response(self):
        with tempfile.TemporaryDirectory() as directory:
            spec = Path(directory) / "openapi.json"
            spec.write_text('{"openapi": "3.1.0", "paths": {"/test": {}}, '
                            '"components": {"schemas": {"Present": {"type": "object"}}}}')
            valid = subprocess.run([sys.executable, str(SCRIPT), str(spec)], capture_output=True, text=True)
            self.assertEqual(valid.returncode, 0, valid.stderr)
            spec.write_text("404: Not Found")
            invalid = subprocess.run([sys.executable, str(SCRIPT), str(spec)], capture_output=True, text=True)
            self.assertEqual(invalid.returncode, 1)
            self.assertIn("Cannot parse OpenAPI document", invalid.stderr)


if __name__ == "__main__":
    unittest.main()
