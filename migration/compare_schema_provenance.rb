require "yaml"
require "json"

old_path, new_path, inventory_path = ARGV
abort "Usage: ruby compare_schema_provenance.rb OLD_STAINLESS_YAML NEW_STAINLESS_YAML API_DIFF_JSON" unless inventory_path

def load_document(path)
  YAML.load_file(path)
end

def resolve_schema(schemas, name, seen = [])
  return [{}, []] if seen.include?(name)
  schema = schemas[name] || {}
  properties = schema["properties"] || {}
  required = Array(schema["required"])
  Array(schema["allOf"]).each do |part|
    if part["$ref"]
      ref_name = part["$ref"].split("/").last
      ref_properties, ref_required = resolve_schema(schemas, ref_name, seen + [name])
      properties = ref_properties.merge(properties)
      required |= ref_required
    else
      properties = (part["properties"] || {}).merge(properties)
      required |= Array(part["required"])
    end
  end
  [properties, required]
end

def camel_property(name)
  return "type" if name == "type"
  parts = name.split("_")
  parts.shift.to_s + parts.map(&:capitalize).join
end

def nullable?(property)
  return nil unless property
  return true if property["nullable"] == true
  return true if Array(property["type"]).include?("null")
  %w[anyOf oneOf].any? do |key|
    Array(property[key]).any? { |variant| variant["type"] == "null" }
  end
end

def shape(property)
  return nil unless property
  result = {}
  %w[type $ref nullable default enum].each { |key| result[key] = property[key] if property.key?(key) }
  %w[anyOf oneOf].each do |key|
    if property[key]
      result[key] = Array(property[key]).map do |variant|
        variant.select { |k, _| %w[type $ref nullable enum].include?(k) }
      end
    end
  end
  result
end

old_document = load_document(old_path)
new_document = load_document(new_path)
old_schemas = old_document.dig("components", "schemas") || {}
new_schemas = new_document.dig("components", "schemas") || {}
inventory = JSON.parse(File.read(inventory_path))
entries = inventory.fetch("public_model_field_changes").map do |change|
  name = change.fetch("model")
  rust_field = change.fetch("field")
  old_properties, old_required = resolve_schema(old_schemas, name)
  new_properties, new_required = resolve_schema(new_schemas, name)
  field = if old_properties.key?(rust_field) || new_properties.key?(rust_field)
    rust_field
  else
    camel_property(rust_field)
  end
  old_property = old_properties[field]
  new_property = new_properties[field]
  classification = if old_property.nil? && new_property.nil?
    "unmatched: no direct component property found; cause unknown"
  elsif old_property.nil?
    "schema property added; generated/source cause is consistent with input addition"
  elsif new_property.nil?
    "schema property removed; generated/source cause is consistent with input removal"
  elsif old_required.include?(field) && !new_required.include?(field) && nullable?(new_property)
    "schema requiredness changed to optional and nullable"
  elsif nullable?(old_property) && nullable?(new_property) &&
      (old_property["nullable"] != new_property["nullable"] || old_property["type"] != new_property["type"])
    "schema null encoding changed; both inputs permit null, while generated Rust state changed"
  elsif shape(old_property) == shape(new_property) && old_required.include?(field) == new_required.include?(field)
    "schema property and requiredness appear unchanged; generator/patch/source cause unknown"
  else
    "schema property or requiredness changed; inspect recorded shapes"
  end
  {
    "model": name,
    "rust_field": change.fetch("field"),
    "registry_rust_type": change["registry"],
    "current_rust_type": change["current"],
    "schema_property": field,
    "registry_present": !old_property.nil?,
    "current_present": !new_property.nil?,
    "registry_required": old_required.include?(field),
    "current_required": new_required.include?(field),
    "registry_nullable": nullable?(old_property),
    "current_nullable": nullable?(new_property),
    "registry_schema_shape": shape(old_property),
    "current_schema_shape": shape(new_property),
    "classification": classification
  }
end

classification_counts = {}
entries.each do |entry|
  key = entry[:classification] || "missing classification"
  classification_counts[key] = classification_counts.fetch(key, 0) + 1
end

output = {
  "inputs": {
    "registry_schema_vcs_revision": "4a996ba65cffd311faed1994cdd78bc73b9007c1",
    "candidate_schema_source_commit": "7918bf2949d38e201f05ee38729aa9286773a6cc",
    "registry_openapi_version": old_document["openapi"],
    "candidate_openapi_version": new_document["openapi"]
  },
  "provenance_notes": [
    "The registry package VCS revision and current base are compared directly; OpenAPI component properties and allOf requiredness are resolved by this script.",
    "Only matching component properties support schema attribution. Unmatched properties and Rust-only changes remain explicitly unknown.",
    "scripts/generate.sh deletes docs/ only. The report and both reproduction scripts live under migration/; their survival was verified by replaying that cleanup against an isolated fixture.",
    "This report does not prove generator output equivalence or an exhaustive semver surface. The source diff independently shows scripts/generate.sh, fix_nullable_fields.py, and fix_boxed_nullable_fields.py unchanged from the registry VCS revision; fix_untagged_unions.py changed its string-enum identifier sanitizer and may affect enum naming."
  ],
  "field_changes": entries,
  "counts": classification_counts
}
puts JSON.pretty_generate(output)
