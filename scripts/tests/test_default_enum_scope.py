import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from fix_generated_code import remove_default_from_problematic_structs


class DefaultEnumScopeTests(unittest.TestCase):
    def test_neighboring_enum_does_not_remove_valid_default(self):
        for enum_name in ["ConnectorId", "WidgetKind"]:
            for filenames in [("tool.rs", "beta_tool.rs"), ("beta_tool.rs", "tool.rs")]:
                with self.subTest(enum=enum_name, order=filenames), tempfile.TemporaryDirectory() as directory:
                    models = Path(directory)
                    (models / "opaque.rs").write_text("pub enum Opaque {\n    Value(String),\n}\n")
                    source = f"""pub enum {enum_name} {{
    First,
}}
impl Default for {enum_name} {{
    fn default() -> Self {{ Self::First }}
}}
pub enum Container {{
    Opaque(models::Opaque),
}}
"""
                    for filename in filenames:
                        (models / filename).write_text(source)
                    remove_default_from_problematic_structs(models)
                    for filename in filenames:
                        self.assertEqual((models / filename).read_text(), source)
                    remove_default_from_problematic_structs(models)
                    for filename in filenames:
                        self.assertEqual((models / filename).read_text(), source)

    def test_actual_non_default_variant_still_removes_invalid_impl(self):
        with tempfile.TemporaryDirectory() as directory:
            models = Path(directory)
            (models / "opaque.rs").write_text("pub enum Opaque {\n    Value(String),\n}\n")
            path = models / "container.rs"
            path.write_text("""pub enum Container {
    Opaque(models::Opaque),
}
impl Default for Container {
    fn default() -> Self { Self::Opaque(Default::default()) }
}
""")
            remove_default_from_problematic_structs(models)
            self.assertNotIn("impl Default", path.read_text())
            expected = path.read_text()
            remove_default_from_problematic_structs(models)
            self.assertEqual(path.read_text(), expected)


if __name__ == "__main__":
    unittest.main()
