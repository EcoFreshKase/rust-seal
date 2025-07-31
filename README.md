# How to setup OQS

[OQS-rust repo](https://github.com/open-quantum-safe/liboqs-rust)
[OQS-rust docs](https://docs.rs/oqs/0.11.0)

## Setup
To setup the application you need to have a local installation of [liboqs](https://github.com/open-quantum-safe/liboqs). Follow the installtion guide in the documentation fo [liboqs](https://github.com/open-quantum-safe/liboqs).

[Setup docs](https://openquantumsafe.org/liboqs/getting-started.html)

# Todo
- replace `oqs.rs` with a new version of `oqs` where `algorithm` enums have a `FromStr` implementation
    - $\to$ see https://github.com/open-quantum-safe/liboqs-rust/pull/292
-  document all available Signature and KEM Algorithm somewhere