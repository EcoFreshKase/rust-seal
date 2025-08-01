use crate::oqs::convert_str_to_sig_alg;

use anyhow::{Context, Result, ensure};
use clap::builder::ValueParser;
use clap::{Arg, Command, ValueHint};
use oqs::sig::{Algorithm, Sig};

use std::path::PathBuf;

pub struct CliArgs {
    pub file_path: PathBuf,
    pub signature: Sig,
}

fn validate_signature_algorithm(algorithm: &str) -> Result<Algorithm> {
    let parsed = convert_str_to_sig_alg(algorithm)
        .context(format!("Invalid signature algorithm: {}", algorithm))?;

    Ok(parsed)
}

pub fn get_args() -> Result<CliArgs> {
    let file_path_arg: Arg = Arg::new("file_path")
        .help("Path to the file to sign")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_name("FILE_PATH")
        .value_parser(clap::value_parser!(PathBuf));

    let signature_algorithm_arg: Arg = Arg::new("signature_algorithm")
        .long("signature-algorithm")
        .short('s')
        .help("Signature algorithm to use")
        .default_value("Dilithium2")
        .value_name("SIGNATURE_ALGORITHM")
        .value_parser(ValueParser::new(validate_signature_algorithm));

    let cmd = Command::new("rust-seal")
        .author("<YOUR_NAME>, <myMail>")
        .version("0.0.1")
        .about("Rust Seal")
        .arg(file_path_arg)
        .arg(signature_algorithm_arg);

    let arg_matches = cmd.get_matches();

    let file_path = arg_matches
        .get_one::<PathBuf>("file_path")
        .context("File path argument is invalid")
        .and_then(|path| {
            ensure!(
                path.is_file(),
                "The specified file does not exist or is not a file"
            );
            Ok(path)
        })?;

    let signature = arg_matches
        .get_one::<Algorithm>("signature_algorithm")
        .context("Signature algorithm argument is invalid")
        .and_then(|algorithm| {
            Sig::new(*algorithm).context("Signature algorithm argument is invalid")
        })?;

    Ok(CliArgs {
        file_path: file_path.to_path_buf(),
        signature,
    })
}
