import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from box_response_errors import patch_constructors, patch_directory, patch_module


class ResponseErrorPatchTests(unittest.TestCase):
    def test_shared_variant_and_idempotence(self):
        source = "pub enum Error<T> { Io(std::io::Error), ResponseError(ResponseContent<T>) }"
        patched = patch_module(source)
        self.assertIn("ResponseError(Box<ResponseContent<T>>)", patched)
        self.assertIn("Io(std::io::Error)", patched)
        self.assertEqual(patch_module(patched), patched)

    def test_different_endpoints_and_payload_types(self):
        for endpoint, model in [("create_eval_run", "CreateEvalRunError"), ("fetch_widget", "WidgetFailure")]:
            with self.subTest(endpoint=endpoint):
                source = f"""fn {endpoint}() -> Result<(), Error<{model}>> {{
    Err(Error::ResponseError(ResponseContent {{ status, content, entity }}))
}}"""
                patched = patch_constructors(source)
                self.assertEqual(patched, source.replace(
                    "ResponseContent { status, content, entity }",
                    "Box::new(ResponseContent { status, content, entity })",
                ))
                self.assertEqual(patch_constructors(patched), patched)

    def test_unrelated_constructors_and_match_arms_unchanged(self):
        source = """Error::Io(e);
ResponseContent { status, content, entity };
match error { Error::ResponseError(e) => e.status, _ => unreachable!() }
"""
        self.assertEqual(patch_constructors(source, allow_patterns=True), source)

    def test_bound_payload_is_rejected(self):
        with self.assertRaisesRegex(ValueError, "Unsupported"):
            patch_constructors("Err(Error::ResponseError(local_var_error))")

    def test_multiline_constructor(self):
        source = "Error::ResponseError(\nResponseContent {\nstatus,\ncontent,\nentity,\n}\n)"
        self.assertEqual(patch_constructors(source),
                         "Error::ResponseError(\nBox::new(ResponseContent {\nstatus,\ncontent,\nentity,\n})\n)")

    def test_changed_generator_shape_fails_before_writes(self):
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            module = apis / "mod.rs"
            original = "pub enum Error<T> { ResponseError(ResponseContent<T>) }"
            module.write_text(original)
            (apis / "widgets_api.rs").write_text(
                "Error::ResponseError(ResponseContent { entity: { nested() } })")
            with self.assertRaisesRegex(ValueError, "Unsupported"):
                patch_directory(apis)
            self.assertEqual(module.read_text(), original)

    def test_directory_patch_is_idempotent_and_preserves_match_patterns(self):
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "mod.rs").write_text(
                "pub enum Error<T> { ResponseError(ResponseContent<T>) }\n"
                "match error { Error::ResponseError(e) => e.status }")
            for name in ["evals_api.rs", "widgets_api.rs"]:
                (apis / name).write_text(
                    "Err(Error::ResponseError(ResponseContent { status, content, entity }))")
            patch_directory(apis)
            expected = {path: path.read_text() for path in apis.glob("*.rs")}
            self.assertIn("Error::ResponseError(e) => e.status", expected[apis / "mod.rs"])
            for name in ["evals_api.rs", "widgets_api.rs"]:
                self.assertIn("Box::new(ResponseContent", expected[apis / name])
            patch_directory(apis)
            self.assertEqual({path: path.read_text() for path in apis.glob("*.rs")}, expected)

    def test_missing_shared_variant_fails(self):
        with self.assertRaisesRegex(ValueError, "Expected one"):
            patch_module("pub enum Error<T> { Other(T) }")


if __name__ == "__main__":
    unittest.main()
