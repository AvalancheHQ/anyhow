use anyhow::{Context, Result};
use std::io;

fn main() {
    divan::main();
}

// Helper function to create an IO error
fn make_io_error() -> io::Error {
    io::Error::new(io::ErrorKind::NotFound, "file not found")
}

// Benchmark adding context to errors
#[divan::bench]
fn context_static_str() -> Result<()> {
    Err(make_io_error()).context("Failed to read file")?;
    Ok(())
}

#[divan::bench]
fn context_string() -> Result<()> {
    Err(make_io_error()).context(format!("Failed to read file"))?;
    Ok(())
}

#[divan::bench]
fn with_context_closure() -> Result<()> {
    let path = "/tmp/test.txt";
    Err(make_io_error()).with_context(|| format!("Failed to read file: {}", path))?;
    Ok(())
}

#[divan::bench]
fn with_context_expensive_string() -> Result<()> {
    Err(make_io_error()).with_context(|| {
        let mut msg = String::from("Failed to read file: ");
        msg.push_str("/very/long/path/to/some/file.txt");
        msg
    })?;
    Ok(())
}

// Benchmark multiple context layers
#[divan::bench]
fn multiple_context_layers() -> Result<()> {
    Err(make_io_error())
        .context("Failed to read config")
        .context("Failed to initialize application")
        .context("Startup failed")?;
    Ok(())
}

// Benchmark context vs direct error creation
#[divan::bench]
fn context_vs_anyhow() -> Result<()> {
    Err(make_io_error()).context("Failed to read file")?;
    Ok(())
}

#[divan::bench]
fn direct_anyhow_error() -> Result<()> {
    anyhow::bail!("Failed to read file: {}", make_io_error());
}
