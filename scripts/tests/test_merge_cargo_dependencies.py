import sys
import tempfile
import tomllib
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from merge_cargo_dependencies import update_cargo


class MergeCargoDependenciesTests(unittest.TestCase):
    def run_merge(self, curated_text, generated_text):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        curated = root / "Cargo.toml"
        generated = root / "Cargo.toml.generated"
        curated.write_text(curated_text)
        generated.write_text(generated_text)
        update_cargo(curated, generated)
        return curated.read_text(), tomllib.loads(curated.read_text())

    def test_keeps_curated_policy_and_unions_generated_features(self):
        text, parsed = self.run_merge(
            '''[dependencies]
reqwest = { version = "^0.13.5", default-features = false, features = ["json", "multipart", "query"] }
reqwest-middleware = { version = "^0.5.2", features = ["json", "query"] }
renamed = { package = "actual-crate", version = "^2", optional = true, features = ["curated"] }
''',
            '''[dependencies]
reqwest = { version = "^0.12", default-features = false, features = ["json", "stream", "generated-only"] }
reqwest-middleware = { version = "^0.4", features = ["json", "multipart"] }
renamed = { package = "actual-crate", version = "2.1", optional = true, features = ["generated"] }
''',
        )
        self.assertEqual("^0.13.5", parsed["dependencies"]["reqwest"]["version"])
        self.assertFalse(parsed["dependencies"]["reqwest"]["default-features"])
        self.assertEqual(
            ["json", "multipart", "query", "stream", "generated-only"],
            parsed["dependencies"]["reqwest"]["features"],
        )
        self.assertEqual("^0.5.2", parsed["dependencies"]["reqwest-middleware"]["version"])
        self.assertEqual(
            ["json", "query", "multipart"],
            parsed["dependencies"]["reqwest-middleware"]["features"],
        )
        self.assertEqual("actual-crate", parsed["dependencies"]["renamed"]["package"])
        self.assertTrue(parsed["dependencies"]["renamed"]["optional"])
        self.assertEqual(["curated", "generated"], parsed["dependencies"]["renamed"]["features"])
        self.assertIn('version = "^0.13.5"', text)

    def test_generated_only_features_and_shorthand_dependencies_survive(self):
        _, parsed = self.run_merge(
            '''[dependencies]
existing = "^1.2"
''',
            '''[dependencies]
existing = { version = "1.4", features = ["new-feature"] }
generated-only = "3.0"
''',
        )
        self.assertEqual("^1.2", parsed["dependencies"]["existing"]["version"])
        self.assertEqual(["new-feature"], parsed["dependencies"]["existing"]["features"])
        self.assertEqual("3.0", parsed["dependencies"]["generated-only"])

    def test_merge_is_byte_idempotent(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            curated = root / "Cargo.toml"
            generated = root / "Cargo.toml.generated"
            curated.write_text('[dependencies]\nreqwest = { version = "^0.13.5", features = ["query"] }\n')
            generated.write_text('[dependencies]\nreqwest = { version = "0.12", features = ["json"] }\n')
            update_cargo(curated, generated)
            first = curated.read_bytes()
            update_cargo(curated, generated)
            self.assertEqual(first, curated.read_bytes())

    def test_conflicting_package_mapping_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "conflicting package mappings.*renamed"):
            self.run_merge(
                '[dependencies]\nrenamed = { package = "first", version = "1" }\n',
                '[dependencies]\nrenamed = { package = "second", version = "1" }\n',
            )

    def test_conflicting_source_mapping_fails_closed(self):
        with self.assertRaisesRegex(ValueError, "conflicting source mappings.*probe"):
            self.run_merge(
                '[dependencies]\nprobe = { git = "https://example.test/one", version = "1" }\n',
                '[dependencies]\nprobe = { git = "https://example.test/two", version = "1" }\n',
            )

    def test_generated_package_conflicts_with_implicit_curated_package(self):
        with self.assertRaisesRegex(ValueError, "conflicting package mappings.*probe"):
            self.run_merge(
                '[dependencies]\nprobe = "^1"\n',
                '[dependencies]\nprobe = { package = "different", version = "1" }\n',
            )

    def test_generated_git_source_conflicts_with_implicit_registry_source(self):
        with self.assertRaisesRegex(ValueError, "conflicting source mappings.*probe"):
            self.run_merge(
                '[dependencies]\nprobe = "^1"\n',
                '[dependencies]\nprobe = { git = "https://example.test/new", version = "1" }\n',
            )

    def test_generated_workspace_source_conflicts_with_implicit_registry_source(self):
        with self.assertRaisesRegex(ValueError, "conflicting source mappings.*probe"):
            self.run_merge(
                '[dependencies]\nprobe = "^1"\n',
                '[dependencies]\nprobe = { workspace = true }\n',
            )

    def test_effective_optional_and_default_feature_policy_is_preserved(self):
        text, parsed = self.run_merge(
            '[dependencies]\nprobe = "^1"\n',
            '[dependencies]\nprobe = { version = "1", optional = false, default-features = true, features = ["generated"] }\n',
        )
        value = parsed["dependencies"]["probe"]
        self.assertEqual("^1", value["version"])
        self.assertNotIn("optional", value)
        self.assertNotIn("default-features", value)
        self.assertEqual(["generated"], value["features"])
        self.assertNotIn("package =", text)

    def test_generated_optional_or_default_feature_change_conflicts_with_implicit_defaults(self):
        for field, value in (("optional", "true"), ("default-features", "false")):
            with self.subTest(field=field):
                with self.assertRaisesRegex(ValueError, f"conflicting effective {field} policy.*probe"):
                    self.run_merge(
                        '[dependencies]\nprobe = "^1"\n',
                        f'[dependencies]\nprobe = {{ version = "1", {field} = {value} }}\n',
                    )


if __name__ == "__main__":
    unittest.main()
