import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from fix_generator_artifacts import fix_content, normalize_identifiers
from fix_helper_impl_mismatches import remove_mismatched_impls


class GeneratorArtifactTests(unittest.TestCase):
    def test_normalizes_only_generated_enum_names(self):
        path = Path("example.rs")
        source = "pub enum Beta_Misalignment { Text, }\nimpl Beta_Misalignment {}\n"
        self.assertEqual(
            "pub enum BetaMisalignment { Text, }\nimpl BetaMisalignment {}\n",
            normalize_identifiers({path: source})[path],
        )

    def test_lowercase_type_keeps_field_and_module_names(self):
        own = Path("delegation.rs")
        other = Path("other.rs")
        files = {
            own: "pub enum delegation { Null, }\nimpl delegation {}\n",
            other: "pub mod delegation;\npub delegation: models::delegation,\n",
        }
        fixed = normalize_identifiers(files)
        self.assertIn("pub enum Delegation", fixed[own])
        self.assertEqual("pub mod delegation;\npub delegation: models::Delegation,\n", fixed[other])

    def test_removes_duplicate_variant_and_rename(self):
        source = (
            "pub enum Example {\n"
            "    TextVariant(String),\n    TextVariant(String),\n"
            "    #[serde(rename = \"1000_tokens\")]\n"
            "    #[serde(rename = \"1000Tokens\")]\n"
            "    Variant1000Tokens,\n}\n"
        )
        fixed = fix_content(source)
        self.assertEqual(1, fixed.count("TextVariant(String),"))
        self.assertIn('rename = "1000_tokens"', fixed)
        self.assertNotIn('rename = "1000Tokens"', fixed)

    def test_stale_default_impl_is_written_as_changed(self):
        source = (
            "pub enum Example { Existing(String), }\n"
            "impl Default for Example { fn default() -> Self { Self::Missing(String::new()) } }\n"
        )
        fixed, changed = remove_mismatched_impls(source)
        self.assertTrue(changed)
        self.assertNotIn("Self::Missing", fixed)


if __name__ == "__main__":
    unittest.main()
