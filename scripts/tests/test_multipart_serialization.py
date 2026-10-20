import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from fix_multipart_serialization import fix_spec


def api_source(functions):
    routed = []
    for function in functions:
        opening = function.find("{")
        route = (
            '\n    let uri_str = format!("{}/fixture", configuration.base_path);\n'
            '    let _request = configuration.client.request(reqwest::Method::POST, &uri_str);'
        )
        routed.append(function[:opening + 1] + route + function[opening + 1:])
    return "use super::{Error, configuration, ContentType};\n\n" + "\n".join(routed)


class MultipartSerializationTests(unittest.TestCase):
    def run_fixer(self, spec, functions):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        apis = Path(temporary.name)
        (apis / "fixture_api.rs").write_text(api_source(functions))
        result = fix_spec(spec, apis)
        first_output = (apis / "fixture_api.rs").read_text()
        fix_spec(spec, apis)
        self.assertEqual(first_output, (apis / "fixture_api.rs").read_text())
        return result, first_output

    @staticmethod
    def operation(operation_id, properties, encoding=None):
        return {
            "openapi": "3.1.0",
            "paths": {
                "/fixture": {
                    "post": {
                        "operationId": operation_id,
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {"type": "object", "properties": properties},
                                    **({"encoding": encoding} if encoding is not None else {}),
                                }
                            }
                        },
                    }
                }
            },
        }

    def test_default_array_is_repeated_in_schema_order_and_empty_vector_emits_nothing(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    let mut multipart_form = reqwest::multipart::Form::new();
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text(
            "values",
            param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string(),
        );
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn("for item in param_value {", source)
        self.assertIn('"values"', source)
        self.assertIn("serde_json::to_value(item)?", source)
        self.assertNotIn('.join(",")', source)
        self.assertIn("if let Some(param_value) = p_form_values", source)

    def test_explicit_form_nonexploded_array_keeps_comma_delimiter(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
            {"values": {"style": "form", "explode": False, "contentType": "application/json"}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn('join(",")', source)
        self.assertIn("if !param_value.is_empty()", source)
        self.assertIn("None,", source)
        self.assertNotIn('Some("application/json")', source)

    def test_explicit_array_content_type_is_applied_to_each_part(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
            {"values": {"contentType": "application/json"}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn('Some("application/json")', source)
        self.assertIn("for item in param_value", source)

    def test_required_scalar_content_type_patch_is_idempotent(self):
        spec = self.operation(
            "sendSdp",
            {"sdp": {"type": "string"}},
            {"sdp": {"contentType": "application/sdp"}},
        )
        function = '''pub async fn send_sdp(configuration: &configuration::Configuration, sdp: &str) -> Result<(), Error<()>> {
    let p_form_sdp = sdp;
    multipart_form = multipart_form.text("sdp", p_form_sdp.to_string());
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn('Some("application/sdp")', source)
        self.assertIn('"sdp",', source)

    def test_object_default_and_explicit_media_types_are_preserved(self):
        spec = {
            "openapi": "3.1.0",
            "components": {"schemas": {"Expiry": {"type": "object", "properties": {"seconds": {"type": "integer"}}}}},
            "paths": {
                "/fixture": {
                    "post": {
                        "operationId": "sendObjects",
                        "requestBody": {
                            "content": {
                                "multipart/form-data": {
                                    "schema": {"type": "object", "properties": {
                                        "expires_after": {"$ref": "#/components/schemas/Expiry"},
                                        "session": {"type": "object", "properties": {"mode": {"type": "string"}}},
                                    }},
                                    "encoding": {"session": {"contentType": "application/json"}},
                                }
                            }
                        },
                    }
                }
            },
        }
        function = '''pub async fn send_objects(configuration: &configuration::Configuration, expires_after: Option<models::Expiry>, session: Option<models::Session>) -> Result<(), Error<()>> {
    let p_form_expires_after = expires_after;
    let p_form_session = session;
    if let Some(param_value) = p_form_expires_after {
        multipart_form = multipart_form.text("expires_after", param_value.to_string());
    }
    if let Some(param_value) = p_form_session {
        multipart_form = multipart_form.text("session", param_value.to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn('"expires_after",', source)
        self.assertIn('"session",', source)
        self.assertIn('Some("application/json")', source)
        self.assertIn("serde_json::to_value(param_value)?", source)

    def test_object_scalar_union_uses_json_only_for_object_values(self):
        spec = self.operation(
            "sendValue",
            {"value": {"anyOf": [
                {"type": "string", "enum": ["auto"]},
                {"type": "object", "properties": {"mode": {"type": "string"}}},
                {"type": "null"},
            ]}},
        )
        function = '''pub async fn send_value(configuration: &configuration::Configuration, value: Option<models::Value>) -> Result<(), Error<()>> {
    let p_form_value = value;
    if let Some(param_value) = p_form_value {
        multipart_form = multipart_form.text("value", param_value.to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn("serde_json::to_value(param_value)?", source)
        self.assertIn("add_schema_value_to_form", source)

    def test_recursive_object_reference_and_nullable_type_list_are_supported(self):
        spec = {
            "openapi": "3.1.0",
            "components": {"schemas": {
                "Recursive": {"type": "object", "properties": {
                    "child": {"$ref": "#/components/schemas/Recursive"},
                }},
            }},
            "paths": {
                "/fixture": {
                    "post": {
                        "operationId": "sendObjects",
                        "requestBody": {"content": {"multipart/form-data": {
                            "schema": {"type": "object", "properties": {
                                "value": {"$ref": "#/components/schemas/Recursive"},
                                "label": {"type": ["string", "null"]},
                            }},
                        } }},
                    }
                }
            },
        }
        function = '''pub async fn send_objects(configuration: &configuration::Configuration, value: Option<models::Recursive>, label: Option<String>) -> Result<(), Error<()>> {
    let p_form_value = value;
    let p_form_label = label;
    if let Some(param_value) = p_form_value {
        multipart_form = multipart_form.text("value", param_value.to_string());
    }
    if let Some(param_value) = p_form_label {
        multipart_form = multipart_form.text("label", param_value.to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn('"value",', source)
        self.assertIn("serde_json::to_value(param_value)?", source)

    def test_array_of_objects_emits_repeated_json_parts(self):
        spec = self.operation(
            "sendItems",
            {"items": {"type": "array", "items": {"type": "object", "properties": {"id": {"type": "string"}}}}},
        )
        function = '''pub async fn send_items(configuration: &configuration::Configuration, items: Option<Vec<models::Item>>) -> Result<(), Error<()>> {
    let p_form_items = items;
    if let Some(param_value) = p_form_items {
        multipart_form = multipart_form.text("items", param_value.to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn("for item in param_value", source)
        self.assertIn('None,', source)

    def test_unsupported_array_style_fails_with_operation_and_field(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
            {"values": {"style": "deepObject"}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "fixture_api.rs").write_text(api_source([function]))
            with self.assertRaisesRegex(ValueError, r"POST /fixture \(sendValues\).values.*deepObject"):
                fix_spec(spec, apis)

    def test_file_union_is_reported_without_changing_binary_upload_path(self):
        spec = self.operation(
            "sendFiles",
            {"files": {"oneOf": [
                {"type": "array", "items": {"type": "string", "format": "binary"}},
                {"type": "string", "format": "binary"},
            ]}},
        )
        function = '''pub async fn send_files(configuration: &configuration::Configuration, files: models::Files) -> Result<(), Error<()>> {
    let p_form_files = files;
    multipart_form = multipart_form.text("files", p_form_files.to_string());
    Ok(())
}'''
        result, source = self.run_fixer(spec, [function])
        self.assertEqual(1, len(result["skipped_binary_fields"]))
        self.assertIn("binary or file-union serialization", result["skipped_binary_fields"][0]["reason"])
        self.assertIn('multipart_form.text("files", p_form_files.to_string())', source)

    def test_binary_encoding_override_fails_closed(self):
        spec = self.operation(
            "sendFiles",
            {"file": {"type": "string", "format": "binary"}},
            {"file": {"contentType": "application/pdf"}},
        )
        function = '''pub async fn send_files(configuration: &configuration::Configuration, file: std::path::PathBuf) -> Result<(), Error<()>> {
    let p_form_file = file;
    multipart_form = multipart_helper::add_file_to_form(multipart_form, &p_form_file, "file")?;
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "fixture_api.rs").write_text(api_source([function]))
            with self.assertRaisesRegex(ValueError, "unsupported multipart binary encoding override.*file"):
                fix_spec(spec, apis)

    def test_normalized_field_collision_fails_before_any_write(self):
        spec = self.operation(
            "sendValues",
            {
                "a-b": {"type": "array", "items": {"type": "string"}},
                "a_b": {"type": "array", "items": {"type": "string"}},
            },
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, a_b: Option<Vec<String>>, a_b_2: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_a_b = a_b;
    let p_form_a_b_2 = a_b_2;
    if let Some(param_value) = p_form_a_b {
        multipart_form = multipart_form.text("a-b", param_value.to_string());
    }
    if let Some(param_value) = p_form_a_b_2 {
        multipart_form = multipart_form.text("a_b", param_value.to_string());
    }
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            source = api_source([function])
            (apis / "fixture_api.rs").write_text(source)
            with self.assertRaisesRegex(ValueError, r"collision.*'a-b'.*'a_b'"):
                fix_spec(spec, apis)
            self.assertEqual(source, (apis / "fixture_api.rs").read_text())

    def test_wrong_field_literal_fails_without_partial_write(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("other", param_value.to_string());
    }
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            source = api_source([function])
            (apis / "fixture_api.rs").write_text(source)
            with self.assertRaisesRegex(ValueError, "wire field does not match POST /fixture.*values"):
                fix_spec(spec, apis)
            self.assertEqual(source, (apis / "fixture_api.rs").read_text())

    def test_function_method_and_path_must_match_openapi_before_any_write(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let uri_str = format!("{}/wrong", configuration.base_path);
    let _request = configuration.client.request(reqwest::Method::GET, &uri_str);
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            source = api_source([function])
            (apis / "fixture_api.rs").write_text(source)
            with self.assertRaisesRegex(ValueError, "method mismatch.*POST /fixture"):
                fix_spec(spec, apis)
            self.assertEqual(source, (apis / "fixture_api.rs").read_text())

    def test_wrong_already_fixed_serializer_is_rejected_without_repair(self):
        spec = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
        )
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_helper::add_schema_value_to_form(multipart_form, "values", serde_json::Value::String("wrong".to_string()), None)?;
    }
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            source = api_source([function])
            (apis / "fixture_api.rs").write_text(source)
            with self.assertRaisesRegex(ValueError, "already-fixed multipart serializer does not match expected semantics"):
                fix_spec(spec, apis)
            self.assertEqual(source, (apis / "fixture_api.rs").read_text())

    def test_local_path_item_reference_and_nullable_array_are_supported(self):
        spec = {
            "openapi": "3.1.0",
            "components": {"pathItems": {"Upload": {
                "post": {
                    "operationId": "sendValues",
                    "requestBody": {"content": {"multipart/form-data": {
                        "schema": {"type": "object", "properties": {
                            "values": {"type": ["array", "null"], "items": {"type": "string"}},
                        }},
                    } }},
                }
            }}},
            "paths": {"/fixture": {"$ref": "#/components/pathItems/Upload"}},
        }
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        _, source = self.run_fixer(spec, [function])
        self.assertIn("for item in param_value", source)
        self.assertIn('"values"', source)

    def test_unsupported_root_composition_fails_after_planned_change_without_writes(self):
        good_function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        other_function = '''pub async fn send_other(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        good_operation = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
        )["paths"]["/fixture"]["post"]
        spec = {
            "openapi": "3.1.0",
            "paths": {
                "/fixture": {"post": good_operation},
                "/unsupported": {"post": {
                    "operationId": "sendOther",
                    "requestBody": {"content": {"multipart/form-data": {
                        "schema": {"allOf": [{"type": "object", "properties": {
                            "values": {"type": "array", "items": {"type": "string"}},
                        }}]},
                        "encoding": {},
                    }}}
                }},
            },
        }
        source = api_source([good_function, other_function])
        route = 'format!("{}/fixture", configuration.base_path)'
        first_route = source.find(route)
        second_route = source.find(route, first_route + len(route))
        self.assertNotEqual(-1, second_route)
        source = source[:second_route] + source[second_route:].replace(
            route, 'format!("{}/unsupported", configuration.base_path)', 1
        )
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "fixture_api.rs").write_text(source)
            with self.assertRaisesRegex(
                ValueError,
                r"unsupported multipart root composition for POST /unsupported \(sendOther\): \['allOf'\]",
            ):
                fix_spec(spec, apis)
            self.assertEqual(source, (apis / "fixture_api.rs").read_text())

    def test_composed_multipart_alternative_is_reported_only_for_exact_generated_json_mapping(self):
        spec = {
            "openapi": "3.1.0",
            "components": {"schemas": {
                "CreateVoiceRequest": {"oneOf": [
                    {"type": "object", "properties": {"audio_sample": {"type": "string", "format": "binary"}}},
                ]},
                "CreateVoicePromptRequest": {"type": "object", "properties": {
                    "type": {"type": "string"}, "name": {"type": "string"}, "prompt": {"type": "string"},
                }},
            }},
            "paths": {"/audio/voices": {"post": {
                "operationId": "createVoice",
                "requestBody": {"required": True, "content": {
                    "application/json": {"schema": {"$ref": "#/components/schemas/CreateVoicePromptRequest"}},
                    "multipart/form-data": {"schema": {"$ref": "#/components/schemas/CreateVoiceRequest"}},
                }},
            }}},
        }
        function = '''pub async fn create_voice(configuration: &configuration::Configuration, create_voice_prompt_request: models::CreateVoicePromptRequest) -> Result<(), Error<()>> {
    let p_body_create_voice_prompt_request = create_voice_prompt_request;
    let uri_str = format!("{}/audio/voices", configuration.base_path);
    let req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
    let req_builder = req_builder.json(&p_body_create_voice_prompt_request);
    Ok(())
}'''
        with tempfile.TemporaryDirectory() as directory:
            apis = Path(directory)
            (apis / "audio_api.rs").write_text(
                "use super::{Error, configuration, ContentType};\n" + function
            )
            result = fix_spec(spec, apis)
            self.assertEqual([], result["changes"])
            self.assertEqual(1, len(result["skipped_unsupported_roots"]))
            exclusion = result["skipped_unsupported_roots"][0]
            self.assertEqual("POST /audio/voices", exclusion["operation"])
            self.assertIn("application/json schema #/components/schemas/CreateVoicePromptRequest", exclusion["reason"])
            self.assertIn("no multipart serializer", exclusion["reason"])
            self.assertIn(".json(&p_body_create_voice_prompt_request)", (apis / "audio_api.rs").read_text())

    def test_mixed_json_and_multipart_body_serializers_fail_in_either_order_without_partial_writes(self):
        good_operation = self.operation(
            "sendValues",
            {"values": {"type": "array", "items": {"type": "string"}}},
        )["paths"]["/fixture"]["post"]
        spec = {
            "openapi": "3.1.0",
            "components": {"schemas": {
                "CreateVoiceRequest": {"oneOf": [
                    {"type": "object", "properties": {"audio_sample": {"type": "string", "format": "binary"}}},
                ]},
                "CreateVoicePromptRequest": {"type": "object", "properties": {
                    "type": {"type": "string"}, "name": {"type": "string"}, "prompt": {"type": "string"},
                }},
            }},
            "paths": {
                "/fixture": {"post": good_operation},
                "/audio/voices": {"post": {
                    "operationId": "createVoice",
                    "requestBody": {"required": True, "content": {
                        "application/json": {"schema": {"$ref": "#/components/schemas/CreateVoicePromptRequest"}},
                        "multipart/form-data": {"schema": {"$ref": "#/components/schemas/CreateVoiceRequest"}},
                    }},
                }},
            },
        }
        good_function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>) -> Result<(), Error<()>> {
    let uri_str = format!("{}/fixture", configuration.base_path);
    let _request = configuration.client.request(reqwest::Method::POST, &uri_str);
    let p_form_values = values;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    Ok(())
}'''
        json_call = "    let req_builder = req_builder.json(&p_body_create_voice_prompt_request);\n"
        multipart_call = "    let req_builder = req_builder.multipart(reqwest::multipart::Form::new());\n"
        for order in ("multipart-before-json", "multipart-after-json"):
            with self.subTest(order=order), tempfile.TemporaryDirectory() as directory:
                calls = multipart_call + json_call if order == "multipart-before-json" else json_call + multipart_call
                mixed_function = f'''pub async fn create_voice(configuration: &configuration::Configuration, create_voice_prompt_request: models::CreateVoicePromptRequest) -> Result<(), Error<()>> {{
    let p_body_create_voice_prompt_request = create_voice_prompt_request;
    let uri_str = format!("{{}}/audio/voices", configuration.base_path);
    let req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);
{calls}    Ok(())
}}'''
                source = "use super::{Error, configuration, ContentType};\n" + good_function + "\n" + mixed_function
                apis = Path(directory)
                (apis / "audio_api.rs").write_text(source)
                with self.assertRaisesRegex(
                    ValueError,
                    r"mixed or ambiguous body serialization for POST /audio/voices \(createVoice\)",
                ):
                    fix_spec(spec, apis)
                self.assertEqual(source, (apis / "audio_api.rs").read_text())

    def test_scalar_encoding_headers_and_style_fail_before_any_write(self):
        function = '''pub async fn send_values(configuration: &configuration::Configuration, values: Option<Vec<String>>, label: Option<String>) -> Result<(), Error<()>> {
    let p_form_values = values;
    let p_form_label = label;
    if let Some(param_value) = p_form_values {
        multipart_form = multipart_form.text("values", param_value.to_string());
    }
    if let Some(param_value) = p_form_label {
        multipart_form = multipart_form.text("label", param_value.to_string());
    }
    Ok(())
}'''
        overrides = (
            {"headers": {"X-Test": {"schema": {"type": "string"}}}},
            {"style": "form"},
        )
        for override in overrides:
            with self.subTest(override=override), tempfile.TemporaryDirectory() as directory:
                spec = self.operation(
                    "sendValues",
                    {
                        "values": {"type": "array", "items": {"type": "string"}},
                        "label": {"type": "string"},
                    },
                    {"label": override},
                )
                apis = Path(directory)
                source = api_source([function])
                (apis / "fixture_api.rs").write_text(source)
                message = "headers" if "headers" in override else "scalar style/explode"
                with self.assertRaisesRegex(ValueError, f"unsupported multipart .*{message}.*label"):
                    fix_spec(spec, apis)
                self.assertEqual(source, (apis / "fixture_api.rs").read_text())


if __name__ == "__main__":
    unittest.main()
