use std::path::PathBuf;
use std::{fs::File, io::Write};

use anyhow::{Context, Result};
use oqs::sig::{SecretKey as SigSecretKey, Sig, Signature};

pub fn sign_and_save_file_signature(
    file_path: &PathBuf,
    signature: &Sig,
    secret_key: &SigSecretKey,
) -> Result<()> {
    let signature =
        get_signature(file_path, signature, secret_key).context("Failed to create signature")?;

    // save file
    let signature_file_path = file_path.with_extension("sig");
    let mut file =
        File::create_new(&signature_file_path).context("Failed to create signature file")?;

    file.write_all(signature.as_ref())
        .context("Failed to write signature to file after creation")?;

    Ok(())
}

fn get_signature(
    file_path: &PathBuf,
    signature: &Sig,
    secret_key: &SigSecretKey,
) -> Result<Signature> {
    let file_content =
        std::fs::read(file_path).context("Failed to read file content for signature creation")?;

    signature
        .sign(&file_content, secret_key)
        .context("Failed to sign file content")
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::{TempDir, tempdir};

    use oqs::sig::Sig;
    use oqs::sig::{Algorithm, PublicKey as SigPublicKey, SecretKey as SigSecretKey};

    use crate::cryptography::signature::sign_and_save_file_signature;

    fn prep_test() -> (TempDir, File, PathBuf, Sig, SigPublicKey, SigSecretKey) {
        let dir = tempdir().expect("Failed to create temporary directory");
        let file_path = dir.path().join("test_file.txt");

        let mut file = File::create_new(&file_path).expect("Failed to create test file");
        writeln!(file, "Hello World!").expect("Failed to write to test file");

        let sig = Sig::new(Algorithm::Dilithium2).expect("Failed to create signature algorithm");
        let keypair = sig.keypair().expect("Failed to generate keypair");

        (dir, file, file_path, sig, keypair.0, keypair.1)
    }

    #[test]
    fn test_signature_file_created() {
        let (_dir, _file, file_path, sig, _, secret_key) = prep_test();

        sign_and_save_file_signature(&file_path, &sig, &secret_key).unwrap();

        assert!(file_path.exists());
    }
}
