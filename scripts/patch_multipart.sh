#!/usr/bin/env bash
set -euo pipefail
ROOT=${1:-crates/openai-patched}

echo "[patch] Adding multipart file upload support..."

# 1) Create the multipart helper module if it doesn't exist
HELPER_FILE="$ROOT/src/apis/multipart_helper.rs"
if [ ! -f "$HELPER_FILE" ]; then
  cat > "$HELPER_FILE" << 'EOF'
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
EOF
  echo "[patch] Created multipart_helper.rs"
fi

# 2) Add multipart_helper module to mod.rs if not already present
MOD_FILE="$ROOT/src/apis/mod.rs"
if ! grep -q "pub(crate) mod multipart_helper;" "$MOD_FILE"; then
  # Add at the end, before the last blank line if present
  echo "" >> "$MOD_FILE"
  echo "// Internal helper module for multipart file handling" >> "$MOD_FILE"
  echo "pub(crate) mod multipart_helper;" >> "$MOD_FILE"
  echo "[patch] Added multipart_helper module to mod.rs"
fi

# 3) Fix audio_api.rs
AUDIO_FILE="$ROOT/src/apis/audio_api.rs"
if [ -f "$AUDIO_FILE" ]; then
  # Add multipart_helper import if not present
  if ! grep -q "multipart_helper" "$AUDIO_FILE"; then
    perl -i -pe 's/use super::\{Error, configuration, ContentType\};/use super::{Error, configuration, ContentType, multipart_helper};/' "$AUDIO_FILE"
  fi
  
  # Fix file parameter placeholders
  perl -i -pe 's/let _p_form_file = file; \/\/ TODO: Implement multipart file handling/let p_form_file = file;/' "$AUDIO_FILE"
  
  # Fix TODO comments for file uploads - preserving any following line
  perl -i -0pe 's/    \/\/ TODO: support file upload for .file. parameter\n/    \/\/ Add file to multipart form\n    multipart_form = multipart_helper::add_file_to_form(multipart_form, &p_form_file, "file")?;\n/g' "$AUDIO_FILE"
  
  echo "[patch] Fixed audio_api.rs multipart handling"
fi

# 4) Fix images_api.rs
IMAGES_FILE="$ROOT/src/apis/images_api.rs"
if [ -f "$IMAGES_FILE" ]; then
  # Add multipart_helper import if not present
  if ! grep -q "multipart_helper" "$IMAGES_FILE"; then
    perl -i -pe 's/use super::\{Error, configuration, ContentType\};/use super::{Error, configuration, ContentType, multipart_helper};/' "$IMAGES_FILE"
  fi
  
  # Fix create_image_edit - change image parameter from &str to PathBuf
  perl -i -pe 's/image: &str,/image: std::path::PathBuf,/' "$IMAGES_FILE"
  
  # Replace the image text field with file upload
  perl -i -pe 's/multipart_form = multipart_form\.text\("image", p_form_image\.to_string\(\)\);/\/\/ Add image file to multipart form\n    multipart_form = multipart_helper::add_file_to_form(multipart_form, \&p_form_image, "image")?;/' "$IMAGES_FILE"
  
  # Fix mask parameter
  perl -i -pe 's/let _p_form_mask = mask; \/\/ TODO: Implement multipart file handling/let p_form_mask = mask;/' "$IMAGES_FILE"
  
  # Fix mask TODO
  perl -i -0pe 's/    \/\/ TODO: support file upload for .mask. parameter\n/    \/\/ Add mask file if provided\n    if let Some(mask_path) = p_form_mask {\n        multipart_form = multipart_helper::add_file_to_form(multipart_form, &mask_path, "mask")?;\n    }\n/g' "$IMAGES_FILE"
  
  # Fix create_image_variation
  perl -i -pe 's/let _p_form_image = image; \/\/ TODO: Implement multipart file handling/let p_form_image = image;/' "$IMAGES_FILE"
  
  # Fix image TODO in variation
  perl -i -0pe 's/    \/\/ TODO: support file upload for .image. parameter\n/    \/\/ Add image file to multipart form\n    multipart_form = multipart_helper::add_file_to_form(multipart_form, &p_form_image, "image")?;\n/g' "$IMAGES_FILE"
  
  echo "[patch] Fixed images_api.rs multipart handling"
fi

# 5) Fix files_api.rs
FILES_FILE="$ROOT/src/apis/files_api.rs"
if [ -f "$FILES_FILE" ]; then
  # Add multipart_helper import if not present
  if ! grep -q "multipart_helper" "$FILES_FILE"; then
    perl -i -pe 's/use super::\{Error, configuration, ContentType\};/use super::{Error, configuration, ContentType, multipart_helper};/' "$FILES_FILE"
  fi
  
  # Fix create_file
  perl -i -pe 's/let _p_form_file = file; \/\/ TODO: Implement multipart file handling/let p_form_file = file;/' "$FILES_FILE"
  
  # Fix file TODO
  perl -i -0pe 's/    \/\/ TODO: support file upload for .file. parameter\n/    \/\/ Add file to multipart form\n    multipart_form = multipart_helper::add_file_to_form(multipart_form, &p_form_file, "file")?;\n/g' "$FILES_FILE"
  
  echo "[patch] Fixed files_api.rs multipart handling"
fi

# 6) Fix uploads_api.rs
UPLOADS_FILE="$ROOT/src/apis/uploads_api.rs"
if [ -f "$UPLOADS_FILE" ]; then
  # Add multipart_helper import if not present
  if ! grep -q "multipart_helper" "$UPLOADS_FILE"; then
    perl -i -pe 's/use super::\{Error, configuration, ContentType\};/use super::{Error, configuration, ContentType, multipart_helper};/' "$UPLOADS_FILE"
  fi
  
  # Fix add_upload_part
  perl -i -pe 's/let _p_form_data = data; \/\/ TODO: Implement multipart file handling/let p_form_data = data;/' "$UPLOADS_FILE"
  
  # Fix data TODO
  perl -i -0pe 's/    \/\/ TODO: support file upload for .data. parameter\n/    \/\/ Add data file to multipart form\n    multipart_form = multipart_helper::add_file_to_form(multipart_form, &p_form_data, "data")?;\n/g' "$UPLOADS_FILE"
  
  echo "[patch] Fixed uploads_api.rs multipart handling"
fi

echo "[patch] Multipart file upload support added successfully"