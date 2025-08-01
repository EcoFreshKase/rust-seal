use crate::commands::{sign_file_command, verify_signature_command};
use crate::oqs::convert_str_to_sig_alg;

use anyhow::{Context, Result};
use clap::builder::ValueParser;
use clap::{Arg, Command, ValueHint};
use oqs::sig::Algorithm;

use std::path::PathBuf;

const FILE_PATH_ID: &str = "file_path";
const SIGNATURE_ALGORITHM_ID: &str = "signature_algorithm";
const SIGNATURE_PATH_ID: &str = "signature_path";
const PUBLIC_KEY_PATH_ID: &str = "public_key_path";

const SIGN_SUBCOMMAND_NAME: &str = "sign";
const VERIFY_SUBCOMMAND_NAME: &str = "verify";

fn validate_signature_algorithm(algorithm: &str) -> Result<Algorithm> {
    let parsed = convert_str_to_sig_alg(algorithm)
        .context(format!("Invalid signature algorithm: {algorithm}"))?;

    Ok(parsed)
}

pub fn create_cli() -> Command {
    let file_path_arg: Arg = Arg::new(FILE_PATH_ID)
        .help("Path to the file to sign")
        .value_name("FILE_PATH")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_parser(clap::value_parser!(PathBuf));

    let signature_algorithm_arg: Arg = Arg::new(SIGNATURE_ALGORITHM_ID)
        .long("signature-algorithm")
        .short('s')
        .help("Signature algorithm to use")
        .default_value("Dilithium2")
        .value_name("SIGNATURE_ALGORITHM")
        .value_parser(ValueParser::new(validate_signature_algorithm));

    let signature_path_arg: Arg = Arg::new(SIGNATURE_PATH_ID)
        .help("Path to the .sig file containing the signature")
        .value_name("SIGNATURE_PATH")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_parser(clap::value_parser!(PathBuf));

    let public_key_path_arg: Arg = Arg::new(PUBLIC_KEY_PATH_ID)
        .help("Path to the .pub file containing the public key")
        .value_name("PUBLIC_KEY_PATH")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_parser(clap::value_parser!(PathBuf));

    let sign_cmd = Command::new(SIGN_SUBCOMMAND_NAME)
        .about("Sign a file")
        .arg(&file_path_arg)
        .arg(&signature_algorithm_arg);

    let verify_cmd = Command::new(VERIFY_SUBCOMMAND_NAME)
        .about("Verify a file signature")
        .arg(&file_path_arg)
        .arg(&signature_path_arg)
        .arg(&public_key_path_arg)
        .arg(&signature_algorithm_arg);

    Command::new("rust-seal")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Rust Seal")
        .subcommand(sign_cmd)
        .subcommand(verify_cmd)
}

pub fn start() -> Result<()> {
    let cli = create_cli();

    let matches = cli.get_matches();

    match matches.subcommand() {
        Some((SIGN_SUBCOMMAND_NAME, sub_matches)) => sign_file_command(sub_matches),
        Some((VERIFY_SUBCOMMAND_NAME, sub_matches)) => verify_signature_command(sub_matches),
        _ => {
            unreachable!("Subcommand should always be present");
        }
    }
}
