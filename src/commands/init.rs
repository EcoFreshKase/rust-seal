use anyhow::{Context, Result};
use clap::ArgMatches;

use crate::{
    Config,
    util::{parse_kem_algorithm_arg, parse_signature_algorithm_arg},
};

pub fn init_kem(args: &ArgMatches, config: &mut Config) -> Result<()> {
    let kem = parse_kem_algorithm_arg(args)?;

    config
        .add_kem_algorithm(&kem)
        .context("Failed to add KEM algorithm to configuration")?;

    println!("Initialized KEM algorithm: {}", kem.algorithm());
    Ok(())
}

pub fn init_sig(args: &ArgMatches, config: &mut Config) -> Result<()> {
    let signature = parse_signature_algorithm_arg(args)?;

    config
        .add_signature_algorithm(&signature)
        .context("Failed to add signature algorithm to configuration")?;

    println!("Initialized signature algorithm: {}", signature.algorithm());
    Ok(())
}
