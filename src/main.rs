use rust_seal::cli::get_args;

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    // Read the file at the given path
    match std::fs::read_to_string(&args.file_path) {
        Ok(contents) => println!("{}", contents),
        Err(e) => eprintln!("Failed to read file {}: {}", args.file_path.display(), e),
    }
}
