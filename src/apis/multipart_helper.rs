/// Internal multipart helper functions for file uploads
/// This module provides utilities to handle file uploads in multipart forms

use std::path::{Path, PathBuf};
use reqwest::multipart;

/// Create a multipart Part from a file path
/// Reads the file and creates a Part with the filename preserved
pub fn file_part_from_path(path: &Path) -> Result<multipart::Part, std::io::Error> {
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("file")
        .to_string();
    
    let bytes = std::fs::read(path)?;
    let part = multipart::Part::bytes(bytes)
        .file_name(file_name);
    
    Ok(part)
}

/// Create a multipart Part from a PathBuf
pub fn file_part_from_pathbuf(path: PathBuf) -> Result<multipart::Part, std::io::Error> {
    file_part_from_path(&path)
}

/// Create a multipart Part from bytes with a filename
pub fn file_part_from_bytes(bytes: Vec<u8>, filename: String) -> multipart::Part {
    multipart::Part::bytes(bytes)
        .file_name(filename)
}

/// Add a file to a multipart form
/// This is a convenience function that reads a file and adds it to the form
pub fn add_file_to_form(form: multipart::Form, path: &Path, field_name: &str) -> Result<multipart::Form, std::io::Error> {
    let part = file_part_from_path(path)?;
    Ok(form.part(field_name.to_string(), part))
}

/// Add a PathBuf file to a multipart form
pub fn add_pathbuf_to_form(form: multipart::Form, path: PathBuf, field_name: &str) -> Result<multipart::Form, std::io::Error> {
    add_file_to_form(form, &path, field_name)
}
