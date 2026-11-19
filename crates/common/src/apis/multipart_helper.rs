use timvw_rust_runtime::multipart;
// Internal multipart helper functions. Files are `timvw_rust_runtime::Upload`s: a path is streamed from disk (and
// opened again by a retry), bytes are sent as they are.
use timvw_rust_runtime::Upload;

/// Add a file to a multipart form: the upload's file name (`file` when it has none) and media type.
/// A path is resolved first (its length read from its metadata, an error when it does not exist), so the form
/// is sent with a Content-Length; the file itself is read while the request is sent.
pub async fn add_file_to_form(
    form: multipart::Form,
    file: impl Into<Upload>,
    field_name: &str,
) -> Result<multipart::Form, timvw_rust_runtime::Error> {
    let upload = file.into().resolve().await?;
    Ok(form.part(field_name.to_string(), multipart::Part::upload(upload)))
}

/// Add a schema-serialized multipart value, applying object and explicit media types.
/// Object and array values default to application/json; scalar strings retain the
/// RFC 7578 default unless the schema supplies an explicit contentType override.
pub fn add_schema_value_to_form(
    form: multipart::Form,
    field_name: &str,
    value: serde_json::Value,
    content_type: Option<&str>,
) -> Result<multipart::Form, timvw_rust_runtime::Error> {
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
