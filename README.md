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
- add pqc encryption 
