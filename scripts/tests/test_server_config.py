"""The server's generator configuration keeps the client's model options."""

import json
import sys
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))

from server_config import CLIENT_ONLY, server_config  # noqa: E402


class ServerConfigTests(unittest.TestCase):
    def setUp(self):
        self.client = json.loads((SCRIPTS / "generator-config.json").read_text())
        self.server = server_config(self.client)

    def test_the_model_options_are_the_clients(self):
        for key in ("stringIdSchemas", "stringIdProperties", "stringProperties", "optionalProperties",
                    "fileProperties", "apiFeaturesFile", "webhookModule", "useChrono", "titleNaming"):
            if key in self.client:
                self.assertEqual(self.server[key], self.client[key], key)

    def test_no_client_option_and_the_server_options(self):
        self.assertFalse(CLIENT_ONLY & self.server.keys(), CLIENT_ONLY & self.server.keys())
        self.assertNotIn("preprocessedSpecFile", self.server)
        self.assertEqual(self.server["targets"], ["server"])
        self.assertEqual(self.server["modelsCrate"], "openai-client-base")
        self.assertEqual(self.server["packageName"], "openai-server-base")
        self.assertTrue(self.server["serverDefaultMethods"])


if __name__ == "__main__":
    unittest.main()
