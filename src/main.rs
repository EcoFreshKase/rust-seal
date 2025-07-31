use rust_seal::cli::get_args;
use rust_seal::cryptography::signature::sign_and_save_file_signature;
use rust_seal::error::RustSealError;

fn main() -> Result<(), RustSealError> {
    let args = match get_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let keypair = args
        .signature
        .keypair()
        .map_err(|e| RustSealError::OqsError(e.to_string()))?;

    sign_and_save_file_signature(&args.file_path, &args.signature, &keypair.1)?;

    println!("{}", args.signature.algorithm().name());

    Ok(())
}
