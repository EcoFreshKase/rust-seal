/// Temporary module for OQS algorithm conversion
/// A PR implementing `FromStr` for `oqs::kem::Algorithm` and `oqs::sig::Algorithm` is pending.
/// Until the PR is merged and a new version is released, we this module is used for conversion.
use oqs::kem::Algorithm as KemAlgorithm;
use oqs::sig::Algorithm as SigAlgorithm;

use crate::error::RustSealError;

pub fn convert_str_to_kem_alg(alg: &str) -> Result<KemAlgorithm, RustSealError> {
    match alg {
        "BIKE-L3" => Ok(KemAlgorithm::BikeL3),
        "BIKE-L5" => Ok(KemAlgorithm::BikeL5),
        "Classic-McEliece-348864" => Ok(KemAlgorithm::ClassicMcEliece348864),
        "Classic-McEliece-348864f" => Ok(KemAlgorithm::ClassicMcEliece348864f),
        "Classic-McEliece-460896" => Ok(KemAlgorithm::ClassicMcEliece460896),
        "Classic-McEliece-460896f" => Ok(KemAlgorithm::ClassicMcEliece460896f),
        "Classic-McEliece-6688128" => Ok(KemAlgorithm::ClassicMcEliece6688128),
        "Classic-McEliece-6688128f" => Ok(KemAlgorithm::ClassicMcEliece6688128f),
        "Classic-McEliece-6960119" => Ok(KemAlgorithm::ClassicMcEliece6960119),
        "Classic-McEliece-6960119f" => Ok(KemAlgorithm::ClassicMcEliece6960119f),
        "Classic-McEliece-8192128" => Ok(KemAlgorithm::ClassicMcEliece8192128),
        "Classic-McEliece-8192128f" => Ok(KemAlgorithm::ClassicMcEliece8192128f),
        "HQC-128" => Ok(KemAlgorithm::Hqc128),
        "HQC-192" => Ok(KemAlgorithm::Hqc192),
        "HQC-256" => Ok(KemAlgorithm::Hqc256),
        "Kyber512" => Ok(KemAlgorithm::Kyber512),
        "Kyber768" => Ok(KemAlgorithm::Kyber768),
        "Kyber1024" => Ok(KemAlgorithm::Kyber1024),
        "ML-KEM-512" => Ok(KemAlgorithm::MlKem512),
        "ML-KEM-768" => Ok(KemAlgorithm::MlKem768),
        "ML-KEM-1024" => Ok(KemAlgorithm::MlKem1024),
        "sntrup761" => Ok(KemAlgorithm::NtruPrimeSntrup761),
        "FrodoKEM-640-AES" => Ok(KemAlgorithm::FrodoKem640Aes),
        "FrodoKEM-640-SHAKE" => Ok(KemAlgorithm::FrodoKem640Shake),
        "FrodoKEM-976-AES" => Ok(KemAlgorithm::FrodoKem976Aes),
        "FrodoKEM-976-SHAKE" => Ok(KemAlgorithm::FrodoKem976Shake),
        "FrodoKEM-1344-AES" => Ok(KemAlgorithm::FrodoKem1344Aes),
        "FrodoKEM-1344-SHAKE" => Ok(KemAlgorithm::FrodoKem1344Shake),
        "BIKE-L1" => Ok(KemAlgorithm::BikeL1),
        _ => Err(RustSealError::OqsError(format!(
            "Unsupported KEM algorithm: {}",
            alg
        ))),
    }
}

pub fn convert_str_to_sig_alg(alg: &str) -> Result<SigAlgorithm, RustSealError> {
    match alg {
        "cross-rsdp-128-balanced" => Ok(SigAlgorithm::CrossRsdp128Balanced),
        "cross-rsdp-128-fast" => Ok(SigAlgorithm::CrossRsdp128Fast),
        "cross-rsdp-128-small" => Ok(SigAlgorithm::CrossRsdp128Small),
        "cross-rsdp-192-balanced" => Ok(SigAlgorithm::CrossRsdp192Balanced),
        "cross-rsdp-192-fast" => Ok(SigAlgorithm::CrossRsdp192Fast),
        "cross-rsdp-192-small" => Ok(SigAlgorithm::CrossRsdp192Small),
        "cross-rsdp-256-balanced" => Ok(SigAlgorithm::CrossRsdp256Balanced),
        "cross-rsdp-256-fast" => Ok(SigAlgorithm::CrossRsdp256Fast),
        "cross-rsdp-256-small" => Ok(SigAlgorithm::CrossRsdp256Small),
        "cross-rsdpg-128-balanced" => Ok(SigAlgorithm::CrossRsdpg128Balanced),
        "cross-rsdpg-128-fast" => Ok(SigAlgorithm::CrossRsdpg128Fast),
        "cross-rsdpg-128-small" => Ok(SigAlgorithm::CrossRsdpg128Small),
        "cross-rsdpg-192-balanced" => Ok(SigAlgorithm::CrossRsdpg192Balanced),
        "cross-rsdpg-192-fast" => Ok(SigAlgorithm::CrossRsdpg192Fast),
        "cross-rsdpg-192-small" => Ok(SigAlgorithm::CrossRsdpg192Small),
        "cross-rsdpg-256-balanced" => Ok(SigAlgorithm::CrossRsdpg256Balanced),
        "cross-rsdpg-256-fast" => Ok(SigAlgorithm::CrossRsdpg256Fast),
        "cross-rsdpg-256-small" => Ok(SigAlgorithm::CrossRsdpg256Small),
        "Dilithium2" => Ok(SigAlgorithm::Dilithium2),
        "Dilithium3" => Ok(SigAlgorithm::Dilithium3),
        "Dilithium5" => Ok(SigAlgorithm::Dilithium5),
        "Falcon-512" => Ok(SigAlgorithm::Falcon512),
        "Falcon-1024" => Ok(SigAlgorithm::Falcon1024),
        "MAYO-1" => Ok(SigAlgorithm::Mayo1),
        "MAYO-2" => Ok(SigAlgorithm::Mayo2),
        "MAYO-3" => Ok(SigAlgorithm::Mayo3),
        "MAYO-5" => Ok(SigAlgorithm::Mayo5),
        "ML-DSA-44" => Ok(SigAlgorithm::MlDsa44),
        "ML-DSA-65" => Ok(SigAlgorithm::MlDsa65),
        "ML-DSA-87" => Ok(SigAlgorithm::MlDsa87),
        "SPHINCS+-SHA2-128f-simple" => Ok(SigAlgorithm::SphincsSha2128fSimple),
        "SPHINCS+-SHA2-128s-simple" => Ok(SigAlgorithm::SphincsSha2128sSimple),
        "SPHINCS+-SHA2-192f-simple" => Ok(SigAlgorithm::SphincsSha2192fSimple),
        "SPHINCS+-SHA2-192s-simple" => Ok(SigAlgorithm::SphincsSha2192sSimple),
        "SPHINCS+-SHA2-256f-simple" => Ok(SigAlgorithm::SphincsSha2256fSimple),
        "SPHINCS+-SHA2-256s-simple" => Ok(SigAlgorithm::SphincsSha2256sSimple),
        "SPHINCS+-SHAKE-128f-simple" => Ok(SigAlgorithm::SphincsShake128fSimple),
        "SPHINCS+-SHAKE-128s-simple" => Ok(SigAlgorithm::SphincsShake128sSimple),
        "SPHINCS+-SHAKE-192f-simple" => Ok(SigAlgorithm::SphincsShake192fSimple),
        "SPHINCS+-SHAKE-192s-simple" => Ok(SigAlgorithm::SphincsShake192sSimple),
        "SPHINCS+-SHAKE-256f-simple" => Ok(SigAlgorithm::SphincsShake256fSimple),
        "SPHINCS+-SHAKE-256s-simple" => Ok(SigAlgorithm::SphincsShake256sSimple),
        "OV-Is" => Ok(SigAlgorithm::UovOvIs),
        "OV-Ip" => Ok(SigAlgorithm::UovOvIp),
        "OV-III" => Ok(SigAlgorithm::UovOvIII),
        "OV-V" => Ok(SigAlgorithm::UovOvV),
        "OV-Is-pkc" => Ok(SigAlgorithm::UovOvIsPkc),
        "OV-Ip-pkc" => Ok(SigAlgorithm::UovOvIpPkc),
        "OV-III-pkc" => Ok(SigAlgorithm::UovOvIIIPkc),
        "OV-V-pkc" => Ok(SigAlgorithm::UovOvVPkc),
        "OV-Is-pkc-skc" => Ok(SigAlgorithm::UovOvIsPkcSkc),
        "OV-Ip-pkc-skc" => Ok(SigAlgorithm::UovOvIpPkcSkc),
        "OV-III-pkc-skc" => Ok(SigAlgorithm::UovOvIIIPkcSkc),
        "OV-V-pkc-skc" => Ok(SigAlgorithm::UovOvVPkcSkc),
        _ => Err(RustSealError::OqsError(format!(
            "Unsupported signature algorithm: {}",
            alg
        ))),
    }
}
