import re
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from fix_untagged_unions import (
    create_simple_string_enum,
    detect_untagged_unions,
    fix_file,
)


class StringEnumIdentifierTests(unittest.TestCase):
    def test_dotted_values_use_consistent_identifiers_and_keep_wire_values(self):
        values = [
            "session.output_transcript.delta",
            "response.audio_transcript.done",
            "session.input-audio_buffer.committed",
        ]
        variants = [
            "SessionOutputTranscriptDelta",
            "ResponseAudioTranscriptDone",
            "SessionInputAudioBufferCommitted",
        ]
        source = create_simple_string_enum("EventType", {
            "values": values,
            "default": values[1],
        })
        for value, variant in zip(values, variants):
            with self.subTest(value=value):
                self.assertIn(f'#[serde(rename = "{value}")]\n    {variant},', source)
                self.assertIn(f'EventType::{variant} => "{value}"', source)
        self.assertIn("Self::ResponseAudioTranscriptDone", source)
        self.assertEqual(
            variants,
            re.findall(r"^    (\w+),$", source, re.MULTILINE),
        )

    def test_existing_variant_names_are_preserved(self):
        for value, variant in [
            ("none", "None"), ("auto", "Auto"), ("required", "Required"),
            ("json_object", "JsonObject"), ("json_schema", "JsonSchema"),
            ("server-vad", "ServerVad"), ("alreadyCamelCase", "Alreadycamelcase"),
        ]:
            with self.subTest(value=value):
                source = create_simple_string_enum("Options", {"values": [value]})
                self.assertIn(f"    {variant},", source)
                self.assertIn(f"Self::{variant}", source)
                self.assertIn(f'Options::{variant} => "{value}"', source)

    def test_union_detection_and_file_generation_with_dotted_values(self):
        # Mirrors LiveDecisionEventSource.event_type: known string events plus
        # an arbitrary string fallback. Exercise the same detection and writer
        # that produce the helper enum inside the generated model file.
        values = ["session.output_transcript.delta", "session.output_transcript.done"]
        for composition in ["anyOf", "oneOf"]:
            with self.subTest(composition=composition):
                spec = {"components": {"schemas": {
                    "LiveDecisionEventSource": {
                        "type": "object",
                        "properties": {"event_type": {composition: [
                            {"type": "string", "enum": values},
                            {"type": "string"},
                        ]}},
                    },
                }}}
                unions, enums = detect_untagged_unions(spec)
                name = "LiveDecisionEventSourceEventType"
                with tempfile.TemporaryDirectory() as directory:
                    path = Path(directory) / "live_decision_event_source_event_type.rs"
                    fix_file(path, name, unions[name], enums)
                    source = path.read_text()
                    self.assertIn(f"pub enum {name}TextVariantEnum", source)
                    self.assertIn("TextVariant(LiveDecisionEventSourceEventTypeTextVariantEnum)", source)
                    self.assertIn("Self::SessionOutputTranscriptDelta", source)
                    self.assertNotIn("::Session.", source)
                    for value in values:
                        self.assertIn(f'#[serde(rename = "{value}")]', source)
                    fix_file(path, name, unions[name], enums)
                    self.assertEqual(source, path.read_text())


if __name__ == "__main__":
    unittest.main()
