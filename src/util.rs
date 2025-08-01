use std::{fs::File, io::Write, path::PathBuf};

use anyhow::{Context, Result, ensure};
use clap::ArgMatches;
use oqs::sig::{Algorithm as SigAlgorithm, Sig};

use crate::cli::SIGNATURE_ALGORITHM_ID;

pub fn create_file_with_content(file_path: &PathBuf, content: &[u8]) -> Result<PathBuf> {
    let mut file = File::create_new(file_path).context("Failed to create file")?;
    file.write_all(content)
        .context("Failed to write content to file")?;
    Ok(file_path.to_path_buf())
}

pub fn parse_path_arg<'a>(matches: &'a ArgMatches, id: &str) -> Result<&'a PathBuf> {
    matches
        .get_one::<PathBuf>(id)
        .context(format!("Missing required argument: {id}"))
        .and_then(|path| {
            ensure!(
                path.is_file(),
                format!("The specified path is not a valid file: {}", path.display())
            );
            Ok(path)
        })
}

pub fn parse_signature_algorithm_arg(matches: &ArgMatches) -> Result<Sig> {
    matches
        .get_one::<SigAlgorithm>(SIGNATURE_ALGORITHM_ID)
        .context("Signature algorithm argument is invalid")
        .and_then(|algorithm| {
            Sig::new(*algorithm).context("Signature algorithm argument is invalid")
        })
}
