use crate::Config;
use crate::commands::{init_kem, init_sig, sign_file_command, verify_signature_command};
use crate::oqs::{convert_str_to_kem_alg, convert_str_to_sig_alg};

use anyhow::{Context, Result};
use clap::builder::ValueParser;
use clap::{Arg, Command, ValueHint};
use oqs::kem::Algorithm as KEMAlgorithm;
use oqs::sig::Algorithm as SignatureAlgorithm;

use std::path::PathBuf;

pub const FILE_PATH_ID: &str = "file_path";
pub const SIGNATURE_ALGORITHM_ID: &str = "signature_algorithm";
pub const KEM_ALGORITHM_ID: &str = "kem_algorithm";
pub const SIGNATURE_PATH_ID: &str = "signature_path";
pub const PUBLIC_KEY_PATH_ID: &str = "public_key_path";

const SIGN_SUBCOMMAND_NAME: &str = "sign";
const VERIFY_SUBCOMMAND_NAME: &str = "verify";
const INIT_SUBCOMMAND_NAME: &str = "init";
const SIG_SUBCOMMAND_NAME: &str = "sig";
const KEM_SUBCOMMAND_NAME: &str = "kem";

fn validate_signature_algorithm(algorithm: &str) -> Result<SignatureAlgorithm> {
    let parsed = convert_str_to_sig_alg(algorithm)
        .context(format!("Invalid signature algorithm: {algorithm}"))?;

    Ok(parsed)
}

fn validate_kem_algorithm(algorithm: &str) -> Result<KEMAlgorithm> {
    let parsed =
        convert_str_to_kem_alg(algorithm).context(format!("Invalid KEM algorithm: {algorithm}"))?;

    Ok(parsed)
}

pub fn create_cli() -> Command {
    let sig_algorithm_arg: Arg = Arg::new(SIGNATURE_ALGORITHM_ID)
        .long("signature-algorithm")
        .short('s')
        .help("Specify the signature algorithm to use")
        .value_name("SIGNATURE_ALGORITHM")
        .value_parser(ValueParser::new(validate_signature_algorithm));

    let kem_algorithm_arg: Arg = Arg::new(KEM_ALGORITHM_ID)
        .long("kem-algorithm")
        .short('k')
        .help("Specify the KEM algorithm to use")
        .value_name("KEM_ALGORITHM")
        .value_parser(ValueParser::new(validate_kem_algorithm));

    let file_path_arg: Arg = Arg::new(FILE_PATH_ID)
        .help("Path to the file to sign")
        .value_name("FILE_PATH")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_parser(clap::value_parser!(PathBuf));

    let signature_path_arg: Arg = Arg::new(SIGNATURE_PATH_ID)
        .help("Path to the .sig file containing the signature")
        .long("sig-path")
        .value_name("SIGNATURE_PATH")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_parser(clap::value_parser!(PathBuf));

    let public_key_path_arg: Arg = Arg::new(PUBLIC_KEY_PATH_ID)
        .help("Path to the .pub file containing the public key")
        .long("pub-path")
        .value_name("PUBLIC_KEY_PATH")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_parser(clap::value_parser!(PathBuf));

    let sign_cmd = Command::new(SIGN_SUBCOMMAND_NAME)
        .about("Sign a file")
        .arg_required_else_help(true)
        .arg(&file_path_arg)
        .arg(&sig_algorithm_arg);

    let verify_cmd = Command::new(VERIFY_SUBCOMMAND_NAME)
        .about("Verify a file signature")
        .arg_required_else_help(true)
        .arg(&file_path_arg)
        .arg(&signature_path_arg)
        .arg(&public_key_path_arg)
        .arg(&sig_algorithm_arg);

    let init_cmd = Command::new(INIT_SUBCOMMAND_NAME)
        .about("Initialize Rust Seal Algorithm")
        .arg_required_else_help(true)
        .subcommand(
            Command::new(KEM_SUBCOMMAND_NAME)
                .about("Initialize a Key Exchange Mechanism (KEM) Algorithm")
                .arg_required_else_help(true)
                .arg(&kem_algorithm_arg),
        )
        .subcommand(
            Command::new(SIG_SUBCOMMAND_NAME)
                .about("Initialize a Signature Algorithm")
                .arg_required_else_help(true)
                .arg(&sig_algorithm_arg),
        );

    Command::new("rust-seal")
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Rust Seal")
        .arg_required_else_help(true)
        .subcommand(sign_cmd)
        .subcommand(verify_cmd)
        .subcommand(init_cmd)
}

pub fn start(config: &mut Config) -> Result<()> {
    let cli = create_cli();

    let matches = cli.get_matches();
    match matches.subcommand() {
        Some((SIGN_SUBCOMMAND_NAME, sub_matches)) => sign_file_command(sub_matches, config),
        Some((VERIFY_SUBCOMMAND_NAME, sub_matches)) => verify_signature_command(sub_matches),
        Some((INIT_SUBCOMMAND_NAME, sub_matches)) => match sub_matches.subcommand() {
            Some((KEM_SUBCOMMAND_NAME, sub_matches)) => init_kem(sub_matches, config),
            Some((SIG_SUBCOMMAND_NAME, sub_matches)) => init_sig(sub_matches, config),
            _ => {
                unreachable!("Subcommand should always be present");
            }
        },
        _ => {
            unreachable!("Subcommand should always be present");
        }
    }
}
