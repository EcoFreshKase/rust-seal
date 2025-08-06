# How to setup OQS

[OQS-rust repo](https://github.com/open-quantum-safe/liboqs-rust)
[OQS-rust docs](https://docs.rs/oqs/0.11.0)

## Setup
To setup the application you need to have a local installation of [liboqs](https://github.com/open-quantum-safe/liboqs). Follow the installation guide in the documentation fo [liboqs](https://github.com/open-quantum-safe/liboqs).

[Setup docs](https://openquantumsafe.org/liboqs/getting-started.html)

# Todo
- replace `oqs.rs` with a new version of `oqs` where `algorithm` enums have a `FromStr` implementation
    - $\to$ see https://github.com/open-quantum-safe/liboqs-rust/pull/292
-  document all available Signature and KEM Algorithm somewhere + terminal hints somehow
- add a command to verify own public key (via certificate) and save that instead of the plain pub key
    - $\to$ while verifying the signature it can be assured that the public key was not altered $\to$ makes sure that the whole verifying process is trustable!

# KEM Algorithms
 - BIKE-L3
 - BIKE-L5
 - Classic-McEliece-348864
 - Classic-McEliece-348864f
 - Classic-McEliece-460896
 - Classic-McEliece-460896f
 - Classic-McEliece-6688128
 - Classic-McEliece-6688128f
 - Classic-McEliece-6960119
 - Classic-McEliece-6960119f
 - Classic-McEliece-8192128
 - Classic-McEliece-8192128f
 - HQC-128
 - HQC-192
 - HQC-256
 - Kyber512
 - Kyber768
 - Kyber1024
 - ML-KEM-512
 - ML-KEM-768
 - ML-KEM-1024
 - sntrup761
 - FrodoKEM-640-AES
 - FrodoKEM-640-SHAKE
 - FrodoKEM-976-AES
 - FrodoKEM-976-SHAKE
 - FrodoKEM-1344-AES
 - FrodoKEM-1344-SHAKE
 - BIKE-L1

# Signature Algorithms

 - cross-rsdp-128-balanced
 - cross-rsdp-128-fast
 - cross-rsdp-128-small
 - cross-rsdp-192-balanced
 - cross-rsdp-192-fast
 - cross-rsdp-192-small
 - cross-rsdp-256-balanced
 - cross-rsdp-256-fast
 - cross-rsdp-256-small
 - cross-rsdpg-128-balanced
 - cross-rsdpg-128-fast
 - cross-rsdpg-128-small
 - cross-rsdpg-192-balanced
 - cross-rsdpg-192-fast
 - cross-rsdpg-192-small
 - cross-rsdpg-256-balanced
 - cross-rsdpg-256-fast
 - cross-rsdpg-256-small
 - Dilithium2
 - Dilithium3
 - Dilithium5
 - Falcon-512
 - Falcon-1024
 - MAYO-1
 - MAYO-2
 - MAYO-3
 - MAYO-5
 - ML-DSA-44
 - ML-DSA-65
 - ML-DSA-87
 - SPHINCS+-SHA2-128f-simple
 - SPHINCS+-SHA2-128s-simple
 - SPHINCS+-SHA2-192f-simple
 - SPHINCS+-SHA2-192s-simple
 - SPHINCS+-SHA2-256f-simple
 - SPHINCS+-SHA2-256s-simple
 - SPHINCS+-SHAKE-128f-simple
 - SPHINCS+-SHAKE-128s-simple
 - SPHINCS+-SHAKE-192f-simple
 - SPHINCS+-SHAKE-192s-simple
 - SPHINCS+-SHAKE-256f-simple
 - SPHINCS+-SHAKE-256s-simple
 - OV-Is
 - OV-Ip
 - OV-III
 - OV-V
 - OV-Is-pkc
 - OV-Ip-pkc
 - OV-III-pkc
 - OV-V-pkc
 - OV-Is-pkc-skc
 - OV-Ip-pkc-skc
 - OV-III-pkc-skc
 - OV-V-pkc-skc
