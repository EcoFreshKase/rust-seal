# Rust Seal

A command-line tool for quantum-safe file signing, encryption, and decryption using post-quantum cryptographic algorithms from the Open Quantum Safe (OQS) library.

## About

Rust Seal provides quantum-resistant cryptographic operations for files, including:
- **File Signing**: Create and verify digital signatures using post-quantum signature algorithms
- **File Encryption/Decryption**: Encrypt files using Key Encapsulation Mechanisms (KEM) with symmetric encryption
- **Key Management**: Generate and manage cryptographic keys for various algorithms

This tool uses the [liboqs](https://github.com/open-quantum-safe/liboqs) library to provide resistance against both classical and quantum computer attacks.

## Installation

### Prerequisites

You need a local installation of [liboqs](https://github.com/open-quantum-safe/liboqs). Follow the [setup documentation](https://openquantumsafe.org/liboqs/getting-started.html) for installation instructions.

### Option 1: Install from crates.io

```bash
cargo install --path .
```

### Option 2: Install from Pre-compiled Binary

```bash
git clone https://github.com/EcoFreshKase/rust-seal
cd rust-seal
cargo build --release
```

The binary will be available at `target/release/rust-seal`.

## Usage

### Initialize Key Pairs

Before using rust-seal, you need to generate key pairs for the algorithms you want to use.

#### Initialize Signature Algorithm
```bash
rust-seal init sig --signature-algorithm <ALGORITHM>
```

#### Initialize KEM Algorithm
```bash
rust-seal init kem --kem-algorithm <ALGORITHM>
```

**Examples:**
```bash
rust-seal init sig --signature-algorithm Dilithium2
rust-seal init kem --kem-algorithm Kyber512
```

### File Signing

#### Sign a File
```bash
rust-seal sign <FILE_PATH> --signature-algorithm <ALGORITHM>
```

**Arguments:**
- `<FILE_PATH>`: Path to the file you want to sign
- `--signature-algorithm, -s`: Signature algorithm to use (required)

**Example:**
```bash
rust-seal sign document.txt --signature-algorithm Dilithium2
```

This creates a signature file `document.txt.sig` alongside your original file.

#### Verify a Signature
```bash
rust-seal verify <FILE_PATH> --signature-algorithm <ALGORITHM> [OPTIONS]
```

**Arguments:**
- `<FILE_PATH>`: Path to the file to verify
- `--signature-algorithm, -s`: Signature algorithm used for signing (required)

**Options:**
- `--sig-path`: Path to signature file (default: `<FILE_PATH>.sig`)
- `--pub-path`: Path to public key file (uses configured key if not specified)

**Example:**
```bash
rust-seal verify document.txt --signature-algorithm Dilithium2
rust-seal verify document.txt --signature-algorithm Dilithium2 --sig-path custom.sig
```

### File Encryption/Decryption

#### Encrypt a File
```bash
rust-seal encrypt-file <FILE_PATH> --kem-algorithm <ALGORITHM> [OPTIONS]
```

**Arguments:**
- `<FILE_PATH>`: Path to the file you want to encrypt
- `--kem-algorithm, -k`: KEM algorithm to use (required)

**Options:**
- `--pub-path`: Path to recipient's public key file

**Example:**
```bash
rust-seal encrypt-file secret.txt --kem-algorithm Kyber512
```

This creates:
- `secret.txt.cipher`: The encrypted file
- `secret.txt.ciphertext`: The encapsulated key

#### Decrypt a File
```bash
rust-seal decrypt-file <FILE_PATH> --kem-algorithm <ALGORITHM> [OPTIONS]
```

**Arguments:**
- `<FILE_PATH>`: Path to the encrypted file (`.cipher` file)
- `--kem-algorithm, -k`: KEM algorithm used for encryption (required)

**Options:**
- `--cipher-path`: Path to ciphertext file (default: derived from file path)

**Example:**
```bash
rust-seal decrypt-file secret.txt.cipher --kem-algorithm Kyber512
```

The decrypted file will be saved with the original name (e.g., `secret.txt`). If the file already exists, `-decrypt` will be appended to avoid overwriting.

## Configuration

Rust Seal stores configuration and generated keys in:
- Configuration file: `rust-seal.config.json`
- Keys directory: `keys/`

The configuration file tracks initialized algorithms and their corresponding key file paths.

## Supported Algorithms

### KEM Algorithms
- BIKE-L1, BIKE-L3, BIKE-L5
- Classic-McEliece-348864, Classic-McEliece-348864f
- Classic-McEliece-460896, Classic-McEliece-460896f
- Classic-McEliece-6688128, Classic-McEliece-6688128f
- Classic-McEliece-6960119, Classic-McEliece-6960119f
- Classic-McEliece-8192128, Classic-McEliece-8192128f
- HQC-128, HQC-192, HQC-256
- Kyber512, Kyber768, Kyber1024
- ML-KEM-512, ML-KEM-768, ML-KEM-1024
- sntrup761
- FrodoKEM-640-AES, FrodoKEM-640-SHAKE
- FrodoKEM-976-AES, FrodoKEM-976-SHAKE
- FrodoKEM-1344-AES, FrodoKEM-1344-SHAKE

### Signature Algorithms
- cross-rsdp-128-balanced, cross-rsdp-128-fast, cross-rsdp-128-small
- cross-rsdp-192-balanced, cross-rsdp-192-fast, cross-rsdp-192-small
- cross-rsdp-256-balanced, cross-rsdp-256-fast, cross-rsdp-256-small
- cross-rsdpg-128-balanced, cross-rsdpg-128-fast, cross-rsdpg-128-small
- cross-rsdpg-192-balanced, cross-rsdpg-192-fast, cross-rsdpg-192-small
- cross-rsdpg-256-balanced, cross-rsdpg-256-fast, cross-rsdpg-256-small
- Dilithium2, Dilithium3, Dilithium5
- Falcon-512, Falcon-1024
- MAYO-1, MAYO-2, MAYO-3, MAYO-5
- ML-DSA-44, ML-DSA-65, ML-DSA-87
- SPHINCS+-SHA2-128f-simple, SPHINCS+-SHA2-128s-simple
- SPHINCS+-SHA2-192f-simple, SPHINCS+-SHA2-192s-simple
- SPHINCS+-SHA2-256f-simple, SPHINCS+-SHA2-256s-simple
- SPHINCS+-SHAKE-128f-simple, SPHINCS+-SHAKE-128s-simple
- SPHINCS+-SHAKE-192f-simple, SPHINCS+-SHAKE-192s-simple
- SPHINCS+-SHAKE-256f-simple, SPHINCS+-SHAKE-256s-simple
- OV-Is, OV-Ip, OV-III, OV-V
- OV-Is-pkc, OV-Ip-pkc, OV-III-pkc, OV-V-pkc
- OV-Is-pkc-skc, OV-Ip-pkc-skc, OV-III-pkc-skc, OV-V-pkc-skc

## Examples

### Complete Workflow: Signing a Document

1. Initialize a signature algorithm:
   ```bash
   rust-seal init sig --signature-algorithm Dilithium2
   ```

2. Sign your document:
   ```bash
   rust-seal sign important-document.pdf --signature-algorithm Dilithium2
   ```

3. Verify the signature:
   ```bash
   rust-seal verify important-document.pdf --signature-algorithm Dilithium2
   ```

### Complete Workflow: Encrypting for Another Party

1. Initialize a KEM algorithm:
   ```bash
   rust-seal init kem --kem-algorithm Kyber512
   ```

2. Encrypt a file:
   ```bash
   rust-seal encrypt-file confidential.txt --kem-algorithm Kyber512
   ```

3. Decrypt the file:
   ```bash
   rust-seal decrypt-file confidential.txt.dec --kem-algorithm Kyber512
   ```

## Security Notes

- Keep your private keys secure and never share them
- Public keys can be safely shared with others for verification and encryption
- Always verify signatures from untrusted sources
- Choose appropriate algorithm security levels based on your threat model

## Todos
- replace `oqs.rs` with a new version of `oqs` where `algorithm` enums have a `FromStr` implementation
    - $\to$ see https://github.com/open-quantum-safe/liboqs-rust/pull/292
-  add cli hints when typing algorithms
    - $\to$ [clap-complete](https://crates.io/crates/clap_complete)
- add a command to verify own public key (via certificate) and save that instead of the plain pub key
    - $\to$ while verifying the signature it can be assured that the public key was not altered 
    - $\to$ makes sure that the whole verifying process is trustable!
