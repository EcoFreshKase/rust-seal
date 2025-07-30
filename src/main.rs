use clap::Parser;
use rust_seal::cli::CliArgs;

fn main() -> Result<(), ()> {
    let args = CliArgs::parse();

    // Read the file at the given path
    match std::fs::read_to_string(&args.file_path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Failed to read file {}: {}", args.file_path.display(), e),
    }

    Ok(())
}
