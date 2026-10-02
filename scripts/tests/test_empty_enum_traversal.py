import json
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from fix_empty_enums import detect_empty_tagged_enums


def ref(name):
    return {"$ref": f"#/components/schemas/{name}"}


def object_schema(**properties):
    return {"type": "object", "properties": properties}


def tagged_union():
    return {
        "oneOf": [ref("Leaf")],
        "discriminator": {"propertyName": "kind"},
    }


LEAF = object_schema(kind={"type": "string", "const": "leaf"})
ENUM_INFO = {
    "property_name": "kind",
    "variants": [{"schema": "Leaf", "discriminator_value": "leaf"}],
}


class EmptyEnumTraversalTests(unittest.TestCase):
    def detect(self, schemas):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "spec.json"
            path.write_text(json.dumps({"components": {"schemas": schemas}}))
            return detect_empty_tagged_enums(path)

    def test_self_reference_does_not_hide_later_union(self):
        result = self.detect({
            "Node": object_schema(next=ref("Node"), choice=tagged_union()),
            "Leaf": LEAF,
        })
        self.assertEqual(result, {"NodeChoice": ENUM_INFO})

    def test_recursive_union_through_array_items(self):
        # Mirrors CompoundFilter -> filters/items/oneOf -> CompoundFilter.
        schemas = {
            "CompoundFilter": object_schema(
                kind={"type": "string", "const": "compound"},
                filters={"type": "array", "items": {
                    "oneOf": [ref("Leaf"), ref("CompoundFilter")],
                    "discriminator": {"propertyName": "kind"},
                }},
                choice=tagged_union(),
            ),
            "Leaf": LEAF,
        }
        recursive_info = {
            "property_name": "kind",
            "variants": [
                {"schema": "Leaf", "discriminator_value": "leaf"},
                {"schema": "CompoundFilter", "discriminator_value": "compound"},
            ],
        }
        self.assertEqual(self.detect(schemas), {
            "CompoundFilterFiltersInner": recursive_info,
            "CompoundFilterFilters": recursive_info,
            "CompoundFilterChoice": ENUM_INFO,
        })

    def test_mutual_recursion_keeps_each_root_and_property_context(self):
        result = self.detect({
            "First": object_schema(second=ref("Second"), choice=tagged_union()),
            "Second": object_schema(first=ref("First"), choice=tagged_union()),
            "Leaf": LEAF,
        })
        self.assertEqual(result, {
            name: ENUM_INFO for name in [
                "FirstSecondChoice", "FirstChoice", "SecondFirstChoice", "SecondChoice",
            ]
        })

    def test_composition_reference_cycles_still_find_inline_branches(self):
        for composition in ["allOf", "oneOf", "anyOf"]:
            with self.subTest(composition=composition):
                result = self.detect({
                    "First": {composition: [ref("Second"), object_schema(choice=tagged_union())]},
                    "Second": {composition: [ref("First")]},
                    "Leaf": LEAF,
                })
                names = ["FirstChoice"]
                if composition == "allOf":
                    names.append("FirstAllOfChoice")
                self.assertEqual(result, {name: ENUM_INFO for name in names})

    def test_shared_reference_retains_siblings_roots_arrays_and_allof_alias(self):
        result = self.detect({
            "First": object_schema(left=ref("Choice"), right=ref("Choice")),
            "Second": {"allOf": [object_schema(choice=ref("Choice"))]},
            "List": {"type": "array", "items": ref("Choice")},
            "Choice": tagged_union(),
            "Leaf": LEAF,
        })
        self.assertEqual(result, {
            name: ENUM_INFO for name in [
                "FirstLeft", "FirstRight", "SecondChoice", "SecondAllOfChoice",
                "ListInner", "List", "Choice",
            ]
        })

    def test_recursive_schema_keeps_inline_variant_names(self):
        result = self.detect({
            "Node": object_schema(next=ref("Node"), choice={
                "anyOf": [
                    object_schema(kind={"const": "first"}),
                    object_schema(kind={"enum": ["second"]}),
                ],
                "discriminator": {"propertyName": "kind"},
            }),
        })
        self.assertEqual(result, {"NodeChoice": {
            "property_name": "kind",
            "variants": [
                {"schema": "NodeChoiceAnyOf", "discriminator_value": "first"},
                {"schema": "NodeChoiceAnyOf1", "discriminator_value": "second"},
            ],
        }})


if __name__ == "__main__":
    unittest.main()
