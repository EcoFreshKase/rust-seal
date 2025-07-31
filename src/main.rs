use rust_seal::cli::get_args;

fn main() {
    let args = match get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    println!("{}", args.signature_algorithm.to_string());
}
