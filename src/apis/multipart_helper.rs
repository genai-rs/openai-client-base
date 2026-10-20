use reqwest::multipart;
/// Internal multipart helper functions for file uploads
/// This module provides utilities to handle file uploads in multipart forms
use std::path::{Path, PathBuf};

/// Create a multipart Part from a file path
/// Reads the file and creates a Part with the filename preserved
pub fn file_part_from_path(path: &Path) -> Result<multipart::Part, std::io::Error> {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();

    let bytes = std::fs::read(path)?;
    let part = multipart::Part::bytes(bytes).file_name(file_name);

    Ok(part)
}

/// Create a multipart Part from a PathBuf
#[allow(dead_code)]
pub fn file_part_from_pathbuf(path: PathBuf) -> Result<multipart::Part, std::io::Error> {
    file_part_from_path(&path)
}

/// Create a multipart Part from bytes with a filename
#[allow(dead_code)]
pub fn file_part_from_bytes(bytes: Vec<u8>, filename: String) -> multipart::Part {
    multipart::Part::bytes(bytes).file_name(filename)
}

/// Add a file to a multipart form
/// This is a convenience function that reads a file and adds it to the form
pub fn add_file_to_form(
    form: multipart::Form,
    path: &Path,
    field_name: &str,
) -> Result<multipart::Form, std::io::Error> {
    let part = file_part_from_path(path)?;
    Ok(form.part(field_name.to_string(), part))
}

/// Add a PathBuf file to a multipart form
#[allow(dead_code)]
pub fn add_pathbuf_to_form(
    form: multipart::Form,
    path: PathBuf,
    field_name: &str,
) -> Result<multipart::Form, std::io::Error> {
    add_file_to_form(form, &path, field_name)
}

/// Add a schema-serialized multipart value, applying object and explicit media types.
/// Object and array values default to application/json; scalar strings retain the
/// RFC 7578 default unless the schema supplies an explicit contentType override.
pub fn add_schema_value_to_form(
    form: multipart::Form,
    field_name: &str,
    value: serde_json::Value,
    content_type: Option<&str>,
) -> Result<multipart::Form, reqwest::Error> {
    let default_json = matches!(
        &value,
        serde_json::Value::Object(_) | serde_json::Value::Array(_)
    );
    let explicit_json = content_type.is_some_and(|media_type| {
        media_type
            .split(';')
            .next()
            .is_some_and(|essence| essence.trim().eq_ignore_ascii_case("application/json"))
    });
    let body = if default_json || explicit_json {
        value.to_string()
    } else {
        match value {
            serde_json::Value::String(text) => text,
            other => other.to_string(),
        }
    };
    let media_type = content_type.or_else(|| default_json.then_some("application/json"));
    let part = match media_type {
        Some(media_type) => multipart::Part::text(body).mime_str(media_type)?,
        None => multipart::Part::text(body),
    };
    Ok(form.part(field_name.to_string(), part))
}
