use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{Aead, OsRng},
};
use anyhow::{Result, bail};
use tracing::debug;

pub const AES_KEY_SIZE: usize = 32; // 256 bits

/// Encrypts data using AES-GCM with the provided key.
/// The key must be of size 32 bytes (256 bits).
///
/// ---
///
///
/// Returns the encrypted data combined with the nonce used for encryption.
///
/// Data format: <nonce-length in bytes ; 1 byte><nonce><encrypted_data>
pub fn symmetric_encrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let aes_key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(aes_key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypted_data = cipher
        .encrypt(&nonce, data)
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    Ok(combine_data_and_nonce(&encrypted_data, &nonce))
}

/// Decrypts data using AES-GCM with the provided key.
/// The key must be of size 32 bytes (256 bits).
///
/// ---
///
///
/// Return the decrypted data.
///
/// Data format: <nonce-length in bytes ; 1 byte><nonce><encrypted_data>
///
pub fn symmetric_decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let aes_key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(aes_key);
    let (nonce, data) = extract_data_and_nonce(data).unwrap();

    let decrypted_data = cipher
        .decrypt(nonce.into(), data)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    Ok(decrypted_data)
}

fn combine_data_and_nonce(data: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut combined = Vec::from(&nonce.len().to_le_bytes()[..1]);
    combined.extend_from_slice(nonce);
    combined.extend_from_slice(data);

    print_byte_array("Data", data);
    print_byte_array("Nonce", nonce);
    print_byte_array("Nonce Length", &nonce.len().to_le_bytes()[..1]);
    print_byte_array("Combined", &combined);

    combined
}

fn extract_data_and_nonce(combined: &[u8]) -> Result<(&[u8], &[u8])> {
    if combined.is_empty() {
        bail!("Combined data is empty");
    }

    let nonce_length = combined[0] as usize;
    if combined.len() <= nonce_length + 1 {
        bail!("Invalid nonce length in combined data");
    }

    let nonce = &combined[1..1 + nonce_length];
    let data = &combined[1 + nonce_length..];

    print_byte_array("Extracted Nonce", nonce);
    print_byte_array("Extracted Data", data);

    Ok((nonce, data))
}

fn print_byte_array(pre: &str, data: &[u8]) {
    let bytes = data
        .iter()
        .map(|&b| format!("{b:#04x} "))
        .collect::<String>();
    debug!("{}:\n{}", pre, bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symmetric_encrypt_decrypt() {
        let data = b"Hello, World!";
        let key: &Key<Aes256Gcm> = &[42; 32].into();

        let encrypted_data = symmetric_encrypt(data, key).expect("Encryption failed");
        let decrypted_data = symmetric_decrypt(&encrypted_data, key).expect("Decryption failed");

        assert_eq!(data, &decrypted_data[..]);
    }
}
