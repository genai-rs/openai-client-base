import copy
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from check_openapi_coverage import audit, normalize_path


class OperationCoverageTests(unittest.TestCase):
    def setUp(self):
        self.spec = {
            "paths": {
                "/items/{id}": {"get": {"operationId": "readItem"}},
                "/tagged": {"get": {"operationId": "readTagged", "tags": ["Probe"]}},
            },
            "webhooks": {"event": {"post": {"operationId": "eventReceived"}}},
        }

    def run_audit(self, modules):
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            for name, content in modules.items():
                (apis / name).write_text(content)
            return audit(self.spec, apis, "webhooks_api")

    def test_parameter_names_are_normalized_but_method_and_path_are_kept(self):
        self.assertEqual("/items/{}", normalize_path("/items/{item_id}"))
        result = self.run_audit({
            "default_api.rs": (
                'pub async fn read_item() { let uri_str = format!("{}/items/{item_id}", base); '
                'client.request(reqwest::Method::GET, &uri_str); }\n'
            ),
            "probe_api.rs": (
                'pub async fn read_tagged() { let uri_str = format!("{}/tagged", base); '
                'client.request(reqwest::Method::GET, &uri_str); }\n'
            ),
            "webhooks_api.rs": (
                'pub async fn event_received() { let uri_str = format!("{}/event", base); '
                'client.request(reqwest::Method::POST, &uri_str); }\n'
            ),
        })
        self.assertEqual("pass", result["status"])
        self.assertEqual(2, len(result["matched_normal_operations"]))
        self.assertEqual(["eventReceived"],
                         [entry["operation_id"] for entry in result["expected_webhooks"]])
        self.assertEqual([], result["missing_normal_operations"])

    def test_webhook_cannot_satisfy_a_normal_path_operation(self):
        spec = copy.deepcopy(self.spec)
        spec["paths"] = {}
        spec["paths"]["/event"] = {"post": {"operationId": "normalEvent"}}
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "webhooks_api.rs").write_text(
                'pub async fn event_received() { let uri_str = format!("{}/event", base); '
                'client.request(reqwest::Method::POST, &uri_str); }\n'
            )
            result = audit(spec, apis, "webhooks_api")
        self.assertEqual("fail", result["status"])
        self.assertEqual(["normalEvent"],
                         [entry["operation_id"] for entry in result["missing_normal_operations"]])

    def test_webhook_missing_from_dedicated_module_fails(self):
        result = self.run_audit({
            "default_api.rs": (
                'pub async fn read_item() { let uri_str = format!("{}/items/{id}", base); '
                'client.request(reqwest::Method::GET, &uri_str); }\n'
            ),
        })
        self.assertEqual("fail", result["status"])
        self.assertEqual(["eventReceived"],
                         [entry["operation_id"] for entry in result["missing_webhook_functions"]])

    def test_webhook_function_with_wrong_method_and_path_fails(self):
        spec = copy.deepcopy(self.spec)
        spec["paths"] = {}
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "webhooks_api.rs").write_text(
                'pub async fn event_received() { let uri_str = format!("{}/WRONG", base); '
                'client.request(reqwest::Method::DELETE, &uri_str); }\n'
            )
            result = audit(spec, apis, "webhooks_api")
        self.assertEqual("fail", result["status"])
        self.assertEqual(1, len(result["mismatched_webhook_functions"]))
        mismatch = result["mismatched_webhook_functions"][0]
        self.assertEqual("POST", mismatch["webhook"]["method"])
        self.assertEqual("/event", mismatch["webhook"]["path"])
        self.assertEqual("DELETE", mismatch["generated"][0]["method"])
        self.assertEqual("/WRONG", mismatch["generated"][0]["path"])

    def test_local_path_item_reference_is_resolved_and_audited(self):
        spec = {
            "paths": {"/items/{id}": {"$ref": "#/components/pathItems/item"}},
            "components": {"pathItems": {
                "item": {"get": {"operationId": "readItem"}},
            }},
        }
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "default_api.rs").write_text(
                'pub async fn read_item() { let uri_str = format!("{}/items/{item_id}", base); '
                'client.request(reqwest::Method::GET, &uri_str); }\n'
            )
            result = audit(spec, apis, "webhooks_api")
        self.assertEqual("pass", result["status"])
        self.assertEqual(1, len(result["expected_normal_operations"]))
        self.assertEqual("readItem",
                         result["matched_normal_operations"][0]["operation"]["operation_id"])

    def test_invalid_or_cyclic_path_item_references_fail_closed(self):
        cases = (
            ({"paths": {"/items": {"$ref": "#/components/pathItems/missing"}}},
             "reference target does not exist"),
            ({"paths": {"/items": {"$ref": "#/components/pathItems/first"}},
              "components": {"pathItems": {
                  "first": {"$ref": "#/components/pathItems/second"},
                  "second": {"$ref": "#/components/pathItems/first"},
              }}}, "cyclic Path Item reference"),
            ({"paths": {"/items": {"$ref": "https://example.test/path-item.json"}}},
             "only local JSON Pointer references are supported"),
            ({"paths": {"/items": {"$ref": None}}},
             "Path Item $ref must be a string"),
            ({"paths": {"/items": {"$ref": 42}}},
             "Path Item $ref must be a string"),
        )
        for spec, expected_error in cases:
            with self.subTest(expected_error=expected_error), tempfile.TemporaryDirectory() as directory:
                result = audit(spec, Path(directory), "webhooks_api")
            self.assertEqual("fail", result["status"])
            self.assertEqual(1, len(result["invalid_input_operations"]))
            self.assertIn(expected_error, result["invalid_input_operations"][0]["error"])

    def test_local_webhook_path_item_reference_is_resolved_and_audited(self):
        spec = {
            "paths": {},
            "webhooks": {"event": {"$ref": "#/components/pathItems/event"}},
            "components": {"pathItems": {
                "event": {"post": {"operationId": "eventReceived"}},
            }},
        }
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "webhooks_api.rs").write_text(
                'pub async fn event_received() { let uri_str = format!("{}/event", base); '
                'client.request(reqwest::Method::POST, &uri_str); }\n'
            )
            result = audit(spec, apis, "webhooks_api")
        self.assertEqual("pass", result["status"])
        self.assertEqual(1, len(result["expected_webhooks"]))
        self.assertEqual("eventReceived", result["expected_webhooks"][0]["operation_id"])

    def test_resolved_webhook_reference_with_no_generated_function_fails(self):
        spec = {
            "paths": {},
            "webhooks": {"event": {"$ref": "#/components/pathItems/event"}},
            "components": {"pathItems": {
                "event": {"post": {"operationId": "eventReceived"}},
            }},
        }
        with tempfile.TemporaryDirectory() as directory:
            result = audit(spec, Path(directory), "webhooks_api")
        self.assertEqual("fail", result["status"])
        self.assertEqual(["eventReceived"],
                         [entry["operation_id"] for entry in result["missing_webhook_functions"]])

    def test_invalid_webhook_path_item_references_fail_closed(self):
        cases = (
            ({"webhooks": {"event": {"$ref": "https://example.test/event.json"}}},
             "only local JSON Pointer references are supported"),
            ({"webhooks": {"event": {"$ref": "#/components/pathItems/first"}},
              "components": {"pathItems": {
                  "first": {"$ref": "#/components/pathItems/second"},
                  "second": {"$ref": "#/components/pathItems/first"},
              }}}, "cyclic Path Item reference"),
            ({"webhooks": {"event": {"$ref": None}}},
             "Path Item $ref must be a string"),
            ({"webhooks": {"event": {"$ref": 42}}},
             "Path Item $ref must be a string"),
        )
        for partial_spec, expected_error in cases:
            spec = {"paths": {}, **partial_spec}
            with self.subTest(expected_error=expected_error), tempfile.TemporaryDirectory() as directory:
                result = audit(spec, Path(directory), "webhooks_api")
            self.assertEqual("fail", result["status"])
            self.assertEqual(1, len(result["invalid_webhook_operations"]))
            self.assertIn(expected_error, result["invalid_webhook_operations"][0]["error"])

    def test_webhook_without_operation_id_uses_generator_name_fallback(self):
        spec = copy.deepcopy(self.spec)
        spec["paths"] = {}
        del spec["webhooks"]["event"]["post"]["operationId"]
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "webhooks_api.rs").write_text(
                'pub async fn event_post() { let uri_str = format!("{}/event", base); '
                'client.request(reqwest::Method::POST, &uri_str); }\n'
            )
            result = audit(spec, apis, "webhooks_api")
        self.assertEqual("pass", result["status"])
        self.assertEqual("webhook-name-and-method",
                         result["expected_webhooks"][0]["operation_id_source"])

    def test_missing_path_operation_is_reported(self):
        result = self.run_audit({"probe_api.rs": (
            'pub async fn read_tagged() { let uri_str = format!("{}/tagged", base); '
            'client.request(reqwest::Method::GET, &uri_str); }\n'
        )})
        self.assertEqual("fail", result["status"])
        self.assertEqual(["readItem"],
                         [entry["operation_id"] for entry in result["missing_normal_operations"]])

    def test_duplicate_emission_is_ambiguous_and_fails(self):
        function = ('pub async fn operation() { let uri_str = format!("{}/items/{id}", base); '
                    'client.request(reqwest::Method::GET, &uri_str); }\n')
        result = self.run_audit({"one_api.rs": function, "two_api.rs": function})
        self.assertEqual("fail", result["status"])
        self.assertEqual(1, len(result["ambiguous_normal_operations"]))

    def test_content_type_methods_of_an_operation_match_it(self):
        def function(name):
            return (f'pub async fn {name}() {{ let uri_str = format!("{{}}/items/{{id}}", base); '
                    'client.request(reqwest::Method::GET, &uri_str); }\n')
        result = self.run_audit({
            "default_api.rs": function("read_item") + function("read_item_stream") + function("read_item_json"),
            "probe_api.rs": (
                'pub async fn read_tagged() { let uri_str = format!("{}/tagged", base); '
                'client.request(reqwest::Method::GET, &uri_str); }\n'
            ),
            "webhooks_api.rs": (
                'pub async fn event_received() { let uri_str = format!("{}/event", base); '
                'client.request(reqwest::Method::POST, &uri_str); }\n'
            ),
        })
        self.assertEqual("pass", result["status"])
        item = [m for m in result["matched_normal_operations"] if m["operation"]["operation_id"] == "readItem"][0]
        self.assertEqual("read_item", item["generated"]["function"])
        self.assertEqual(["read_item_stream", "read_item_json"],
                         [entry["function"] for entry in item["content_type_methods"]])

    def test_response_functions_of_an_operation_with_an_apostrophe_match_it(self):
        # The generator escapes the apostrophe as _quote_; with responseFunctions the operation has two functions.
        self.spec["paths"]["/keys"] = {"get": {"operationId": "Listthekeysoftheuser'sorganization"}}

        def function(name, path):
            return (f'pub async fn {name}() {{ let uri_str = format!("{{}}{path}", base); '
                    'client.request(reqwest::Method::GET, &uri_str); }\n')
        name = "listthekeysoftheuser_quote_sorganization"
        result = self.run_audit({
            "default_api.rs": function("read_item", "/items/{id}") + function("read_item_response", "/items/{id}")
            + function(name, "/keys") + function(name + "_response", "/keys"),
            "probe_api.rs": function("read_tagged", "/tagged"),
            "webhooks_api.rs": (
                'pub async fn event_received() { let uri_str = format!("{}/event", base); '
                'client.request(reqwest::Method::POST, &uri_str); }\n'
            ),
        })
        self.assertEqual("pass", result["status"], result)
        item = [m for m in result["matched_normal_operations"] if m["operation"]["path"] == "/keys"][0]
        self.assertEqual(name, item["generated"]["function"])

    def test_a_suffixed_function_in_another_module_is_ambiguous(self):
        def function(name):
            return (f'pub async fn {name}() {{ let uri_str = format!("{{}}/items/{{id}}", base); '
                    'client.request(reqwest::Method::GET, &uri_str); }\n')
        result = self.run_audit({"one_api.rs": function("read_item"), "two_api.rs": function("read_item_stream")})
        self.assertEqual("fail", result["status"])
        self.assertEqual(1, len(result["ambiguous_normal_operations"]))

    def test_unparseable_generated_function_fails_closed(self):
        result = self.run_audit({"default_api.rs": "pub async fn mystery() {}\n"})
        self.assertEqual("fail", result["status"])
        self.assertEqual("mystery", result["unparsed_generated_functions"][0]["function"])


if __name__ == "__main__":
    unittest.main()
