use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn create_error_from_message() -> anyhow::Result<()> {
    Err(anyhow::anyhow!("an error occurred"))
}

#[divan::bench]
fn create_error_from_io_error() -> anyhow::Result<()> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(anyhow::Error::new(io_err))
}

#[divan::bench]
fn create_error_with_context() -> anyhow::Result<()> {
    use anyhow::Context;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).context("failed to open configuration file")
}

#[divan::bench]
fn create_error_with_lazy_context() -> anyhow::Result<()> {
    use anyhow::Context;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    Err(io_err).with_context(|| format!("failed to open file: {}", "config.json"))
}

#[divan::bench]
fn error_formatting() -> String {
    let err = anyhow::anyhow!("an error occurred");
    format!("{}", err)
}

#[divan::bench]
fn error_debug_formatting() -> String {
    let err = anyhow::anyhow!("an error occurred");
    format!("{:?}", err)
}

#[divan::bench]
fn error_chain_iteration() -> usize {
    use anyhow::Context;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: anyhow::Error = anyhow::Error::new(io_err)
        .context("failed to read config")
        .context("application startup failed");
    
    err.chain().count()
}
