use anyhow::{anyhow, bail, Context, Result};
use std::io;

fn main() {
    divan::main();
}

// Benchmark basic error creation patterns
#[divan::bench]
fn create_anyhow_macro() -> Result<()> {
    bail!("Something went wrong");
}

#[divan::bench]
fn create_from_string() -> Result<()> {
    Err(anyhow!("Error message"))
}

#[divan::bench]
fn create_with_format() -> Result<()> {
    let value = 42;
    bail!("Failed to process value: {}", value);
}

#[divan::bench]
fn create_from_io_error() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(anyhow::Error::from(io_err))
}

#[divan::bench]
fn create_from_std_error() -> Result<()> {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    Err(io_err)?;
    Ok(())
}

// Benchmark error propagation with ? operator
#[divan::bench]
fn propagate_with_question_mark() -> Result<()> {
    fn inner() -> Result<()> {
        bail!("inner error");
    }
    inner()?;
    Ok(())
}

// Benchmark error chains
#[divan::bench]
fn create_error_chain() -> Result<()> {
    fn level3() -> Result<()> {
        bail!("level 3 error");
    }
    
    fn level2() -> Result<()> {
        level3().context("level 2 context")?;
        Ok(())
    }
    
    fn level1() -> Result<()> {
        level2().context("level 1 context")?;
        Ok(())
    }
    
    level1()
}
