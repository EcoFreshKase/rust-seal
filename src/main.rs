use anyhow::{Context, Result};

use rust_seal::Config;

fn main() -> Result<()> {
    let mut config = Config::new().context("Failed to load configuration")?;

    rust_seal::cli::start(&mut config)?;
    config.save()?;

    Ok(())
}
