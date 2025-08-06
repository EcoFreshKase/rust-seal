use std::{
    fs::{create_dir_all, read, read_to_string, write},
    path::PathBuf,
};

use anyhow::{Context, Result, bail};
use oqs::{
    kem::{
        Algorithm as KemAlgorithmVariant, Kem, PublicKey as KemPublicKey, SecretKey as KemSecretKey,
    },
    sig::{
        Algorithm as SigAlgorithmVariant, PublicKey as SigPublicKey, SecretKey as SigSecretKey, Sig,
    },
};
use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = "./rust-seal.config.json";
const SIG_KEYS_DIR_PATH: &str = "./keys/sig";
const KEM_KEYS_DIR_PATH: &str = "./keys/kem";

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
#[derive(Default)]
pub struct Config {
    kem_algorithms: Vec<KemAlgorithm>,
    signature_algorithms: Vec<SigAlgorithm>,
}

#[derive(Serialize, Deserialize, Debug)]
struct KemAlgorithm {
    algorithm: KemAlgorithmVariant,
    pub_key_path: PathBuf,
    sec_key_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct SigAlgorithm {
    algorithm: SigAlgorithmVariant,
    pub_key_path: PathBuf,
    sec_key_path: PathBuf,
}

impl Config {
    pub fn new() -> Result<Self> {
        let contents = match read_to_string(CONFIG_FILE_PATH) {
            Ok(contents) => contents,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                write(CONFIG_FILE_PATH, "{}").context("Failed to create default config file")?;
                "{}".to_string()
            }
            Err(err) => return Err(err).context("Failed to read config file"),
        };

        let config: Config =
            serde_json::from_str(&contents).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let json_string =
            serde_json::to_string_pretty(&self).context("Failed to serialize config")?;
        write(CONFIG_FILE_PATH, json_string).context("Failed to write config file")?;
        Ok(())
    }

    pub fn add_kem_algorithm(&mut self, kem: &Kem) -> Result<(KemPublicKey, KemSecretKey)> {
        if self
            .kem_algorithms
            .iter()
            .map(|element| element.algorithm)
            .any(|alg| alg == kem.algorithm())
        {
            bail!("KEM algorithm {} already initialized", kem.algorithm());
        }

        create_dir_all(KEM_KEYS_DIR_PATH).context("Failed to create kem-keys directory")?;

        let (public_key, secret_key) = kem
            .keypair()
            .context("Failed to generate keypair for KEM algorithm")?;
        let pub_key_path = PathBuf::from(format!("{}/{}.pub", KEM_KEYS_DIR_PATH, kem.algorithm()));
        let sec_key_path = PathBuf::from(format!("{}/{}.sec", KEM_KEYS_DIR_PATH, kem.algorithm()));

        write(&pub_key_path, &public_key).context("Failed to save public key to file")?;
        println!("Public key saved to: {}", pub_key_path.display());

        write(&sec_key_path, &secret_key).context("Failed to save secret key to file")?;
        println!("Secret key saved to: {}", sec_key_path.display());

        self.kem_algorithms.push(KemAlgorithm {
            algorithm: kem.algorithm(),
            pub_key_path,
            sec_key_path,
        });

        Ok((public_key, secret_key))
    }

    pub fn add_signature_algorithm(&mut self, sig: &Sig) -> Result<(SigPublicKey, SigSecretKey)> {
        if self
            .signature_algorithms
            .iter()
            .map(|element| element.algorithm)
            .any(|alg| alg == sig.algorithm())
        {
            bail!(
                "Signature algorithm {} already initialized",
                sig.algorithm()
            );
        }

        create_dir_all(SIG_KEYS_DIR_PATH).context("Failed to create sig-keys directory")?;

        let (public_key, secret_key) = sig
            .keypair()
            .context("Failed to generate keypair for signature algorithm")?;
        let pub_key_path = PathBuf::from(format!("{}/{}.pub", SIG_KEYS_DIR_PATH, sig.algorithm()));
        let sec_key_path = PathBuf::from(format!("{}/{}.sec", SIG_KEYS_DIR_PATH, sig.algorithm()));

        write(&pub_key_path, &public_key).context("Failed to save public key to file")?;
        println!("Public key saved to: {}", pub_key_path.display());

        write(&sec_key_path, &secret_key).context("Failed to save secret key to file")?;
        println!("Secret key saved to: {}", sec_key_path.display());

        self.signature_algorithms.push(SigAlgorithm {
            algorithm: sig.algorithm(),
            pub_key_path,
            sec_key_path,
        });

        Ok((public_key, secret_key))
    }

    pub fn get_signature_keys(
        &self,
        algorithm: &SigAlgorithmVariant,
    ) -> Result<(SigPublicKey, SigSecretKey)> {
        self.signature_algorithms
            .iter()
            .find(|alg| &alg.algorithm == algorithm)
            .map(|alg| {
                let public_key =
                    read(&alg.pub_key_path).context("Failed to read public key file")?;
                let secret_key =
                    read(&alg.sec_key_path).context("Failed to read secret key file")?;

                let sig = Sig::new(*algorithm).context(
                    "Failed to create signature algorithm. Algorithm might me disabled.",
                )?;

                Ok((
                    sig.public_key_from_bytes(&public_key)
                        .context(format!("Public key is not a valid key for {algorithm}"))?
                        .to_owned(),
                    sig.secret_key_from_bytes(&secret_key)
                        .context(format!("Secret key is not a valid key for {algorithm}"))?
                        .to_owned(),
                ))
            })
            .ok_or(anyhow::anyhow!(
                "Signature algorithm {} not found",
                algorithm
            ))?
    }

    pub fn get_kem_keys(
        &self,
        algorithm: &KemAlgorithmVariant,
    ) -> Result<(KemPublicKey, KemSecretKey)> {
        self.kem_algorithms
            .iter()
            .find(|alg| &alg.algorithm == algorithm)
            .map(|alg| {
                let public_key =
                    read(&alg.pub_key_path).context("Failed to read public key file")?;
                let secret_key =
                    read(&alg.sec_key_path).context("Failed to read secret key file")?;

                let kem = Kem::new(*algorithm)
                    .context("Failed to create KEM algorithm. Algorithm might me disabled.")?;

                Ok((
                    kem.public_key_from_bytes(&public_key)
                        .context(format!("Public key is not a valid key for {algorithm}"))?
                        .to_owned(),
                    kem.secret_key_from_bytes(&secret_key)
                        .context(format!("Secret key is not a valid key for {algorithm}"))?
                        .to_owned(),
                ))
            })
            .ok_or(anyhow::anyhow!("KEM algorithm {} not found", algorithm))?
    }
}
