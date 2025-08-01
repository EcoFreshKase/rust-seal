use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{Context, Result};

pub fn create_file_with_content(file_path: &PathBuf, content: &[u8]) -> Result<PathBuf> {
    let mut file = File::create_new(file_path).context("Failed to create file")?;
    file.write_all(content)
        .context("Failed to write content to file")?;
    Ok(file_path.to_path_buf())
}
