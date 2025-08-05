use std::path::PathBuf;

use anyhow::{Context, Result, ensure};
use clap::ArgMatches;
use oqs::kem::{Algorithm as KEMAlgorithm, Kem};
use oqs::sig::{Algorithm as SigAlgorithm, Sig};

use crate::cli::{KEM_ALGORITHM_ID, SIGNATURE_ALGORITHM_ID};

pub fn parse_path_arg(matches: &ArgMatches, id: &str) -> Result<PathBuf> {
    matches
        .get_one::<PathBuf>(id)
        .context(format!("Missing required argument: {id}"))
        .and_then(|path| {
            ensure!(
                path.is_file(),
                format!("The specified path is not a valid file: {}", path.display())
            );
            Ok(path.to_owned())
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

pub fn parse_kem_algorithm_arg(matches: &ArgMatches) -> Result<Kem> {
    matches
        .get_one::<KEMAlgorithm>(KEM_ALGORITHM_ID)
        .context("KEM algorithm argument is invalid")
        .and_then(|algorithm| Kem::new(*algorithm).context("KEM algorithm argument is invalid"))
}
