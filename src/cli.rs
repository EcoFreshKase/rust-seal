use crate::oqs::convert_str_to_sig_alg;

use super::error::RustSealError;
use clap::builder::ValueParser;
use clap::{Arg, Command, ValueHint};
use oqs::sig::{Algorithm, Sig};
use std::path::PathBuf;

pub struct CliArgs {
    pub file_path: PathBuf,
    pub signature: Sig,
}

fn validate_signature_algorithm(algorithm: &str) -> Result<Algorithm, std::io::Error> {
    let parsed = convert_str_to_sig_alg(algorithm).map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid signature algorithm",
        )
    })?;

    Ok(parsed)
}

pub fn get_args() -> Result<CliArgs, RustSealError> {
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
        .ok_or_else(|| {
            RustSealError::CliInvalidArgument("File path argument is invalid".to_string())
        })
        .and_then(|path| {
            if !path.is_file() {
                return Err(RustSealError::CliInvalidArgument(
                    "The specified file does not exist or is not a file".to_string(),
                ));
            }
            Ok(path)
        })?;

    let signature = arg_matches
        .get_one::<Algorithm>("signature_algorithm")
        .ok_or_else(|| {
            RustSealError::CliInvalidArgument("Signature algorithm argument is invalid".to_string())
        })
        .and_then(|algorithm| {
            Sig::new(*algorithm).map_err(|_| {
                RustSealError::CliInvalidArgument(
                    "Signature algorithm argument is invalid".to_string(),
                )
            })
        })?;

    Ok(CliArgs {
        file_path: file_path.to_path_buf(),
        signature,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::{File, read_to_string};
    use std::io::Write;
    use tempfile::tempdir;

    use oqs::sig::Algorithm;
    use oqs::sig::Sig;

    use crate::cli::CliArgs;

    #[test]
    fn smoke_test() {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("test_file.txt");

        let mut file = File::create(&file_path).expect("failed to create file");
        writeln!(file, "Hello, world!").expect("failed to write to file");

        let args = CliArgs {
            file_path: file_path.clone(),
            signature: Sig::new(Algorithm::Dilithium2).unwrap(),
        };

        assert_eq!(
            "Hello, world!\n",
            read_to_string(&args.file_path).expect("failed to read file")
        );
    }
}
