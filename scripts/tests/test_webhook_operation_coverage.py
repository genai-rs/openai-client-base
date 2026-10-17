import copy
import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from check_openapi_coverage import audit, normalize_path
from preserve_webhook_api import preserve_webhook_api
from route_webhook_apis import WEBHOOK_API_TAG, route_webhooks


class WebhookRoutingTests(unittest.TestCase):
    def setUp(self):
        self.fixture_path = Path(__file__).parent / "fixtures/webhook_collision/openapi.json"
        self.spec = json.loads(self.fixture_path.read_text())

    def test_routing_changes_only_webhook_tag_metadata_and_is_idempotent(self):
        original = copy.deepcopy(self.spec)
        operation = self.spec["webhooks"]["event"]["post"]
        operation["tags"] = ["OriginalWebhookGroup"]
        original = copy.deepcopy(self.spec)

        self.assertEqual(1, route_webhooks(self.spec))
        self.assertEqual(0, route_webhooks(self.spec))
        self.assertEqual(["OriginalWebhookGroup", WEBHOOK_API_TAG],
                         self.spec["webhooks"]["event"]["post"]["tags"])
        self.assertEqual(original["paths"], self.spec["paths"])
        self.assertEqual(original["components"], self.spec["components"])
        expected_webhook = copy.deepcopy(original["webhooks"])
        expected_webhook["event"]["post"]["tags"].append(WEBHOOK_API_TAG)
        self.assertEqual(expected_webhook, self.spec["webhooks"])

    def test_reserved_tag_on_normal_path_fails_closed(self):
        self.spec["paths"]["/tagged"]["get"]["tags"].append(WEBHOOK_API_TAG)
        with self.assertRaisesRegex(ValueError, "already used by normal path"):
            route_webhooks(self.spec)

    def test_no_webhooks_is_a_noop(self):
        self.spec.pop("webhooks")
        before = copy.deepcopy(self.spec)
        self.assertEqual(0, route_webhooks(self.spec))
        self.assertEqual(before, self.spec)

    def test_webhook_exports_are_reexported_from_the_legacy_module(self):
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "mod.rs").write_text(
                "pub mod default_api;\npub mod open_ai_client_base_webhooks_api;\n"
            )
            (apis / "default_api.rs").write_text("pub async fn read_untagged() {}\n")
            (apis / "open_ai_client_base_webhooks_api.rs").write_text(
                "pub async fn event_received() {}\n"
            )
            preserve_webhook_api(apis)
            preserve_webhook_api(apis)
            exported = (apis / "default_api.rs").read_text()
            self.assertEqual(1, exported.count(
                "pub use super::open_ai_client_base_webhooks_api::*;"
            ))
            self.assertIn("pub async fn read_untagged", exported)
            self.assertIn("pub async fn event_received", (apis / "open_ai_client_base_webhooks_api.rs").read_text())

    def test_legacy_webhook_module_is_created_when_no_path_uses_default_tag(self):
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "mod.rs").write_text("pub mod probe_api;\npub mod open_ai_client_base_webhooks_api;\n")
            (apis / "open_ai_client_base_webhooks_api.rs").write_text("pub async fn event_received() {}\n")
            preserve_webhook_api(apis)
            self.assertTrue((apis / "default_api.rs").exists())
            self.assertIn("pub mod default_api;", (apis / "mod.rs").read_text())


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

    def test_unparseable_generated_function_fails_closed(self):
        result = self.run_audit({"default_api.rs": "pub async fn mystery() {}\n"})
        self.assertEqual("fail", result["status"])
        self.assertEqual("mystery", result["unparsed_generated_functions"][0]["function"])


if __name__ == "__main__":
    unittest.main()
