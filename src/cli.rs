use super::error::RustSealError;
use clap::{Arg, Command, ValueHint};
use std::path::PathBuf;

pub struct CliArgs {
    pub file_path: PathBuf,
}

pub fn get_args() -> Result<CliArgs, RustSealError> {
    let file_path_arg: Arg = Arg::new("file_path")
        .help("Path to the file to sign")
        .required(true)
        .value_hint(ValueHint::FilePath)
        .value_name("FILE_PATH")
        .value_parser(clap::value_parser!(PathBuf));

    let cmd = Command::new("rust-seal")
        .author("<YOUR_NAME>, <myMail>")
        .version("0.0.1")
        .about("Rust Seal")
        .arg(file_path_arg);

    let arg_matches = cmd.get_matches();

    let file_path = match arg_matches.get_one::<PathBuf>("file_path") {
        Some(path) => {
            if !path.is_file() {
                return Err(RustSealError::CliInvalidArgument(
                    "The specified file does not exist or is not a file".to_string(),
                ));
            }
            path.clone()
        }
        None => {
            return Err(RustSealError::CliInvalidArgument(
                "File path argument is invalid".to_string(),
            ));
        }
    };

    Ok(CliArgs {
        file_path: file_path,
    })
}

#[cfg(test)]
mod tests {
    use std::fs::{File, read_to_string};
    use std::io::Write;
    use tempfile::tempdir;

    use crate::cli::CliArgs;

    #[test]
    fn smoke_test() {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("test_file.txt");

        let mut file = File::create(&file_path).expect("failed to create file");
        writeln!(file, "Hello, world!").expect("failed to write to file");

        let args = CliArgs {
            file_path: file_path.clone(),
        };

        assert_eq!(
            "Hello, world!\n",
            read_to_string(&args.file_path).expect("failed to read file")
        );
    }
}
