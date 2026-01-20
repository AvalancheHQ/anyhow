use std::io;

fn main() {
    divan::main();
}

#[divan::bench]
fn add_context_to_result() -> anyhow::Result<()> {
    use anyhow::Context;
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result.context("failed to perform operation")
}

#[divan::bench]
fn add_multiple_contexts() -> anyhow::Result<()> {
    use anyhow::Context;
    let result: Result<(), io::Error> = Err(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    result
        .context("failed to read file")
        .context("configuration load failed")
        .context("application initialization failed")
}

#[divan::bench]
fn downcast_error() -> Option<io::ErrorKind> {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = anyhow::Error::new(io_err);
    
    err.downcast_ref::<io::Error>().map(|e| e.kind())
}

#[divan::bench]
fn downcast_through_context() -> Option<io::ErrorKind> {
    use anyhow::Context;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: anyhow::Error = anyhow::Error::new(io_err)
        .context("operation failed");
    
    err.downcast_ref::<io::Error>().map(|e| e.kind())
}

#[divan::bench]
fn error_source_traversal() -> usize {
    use anyhow::Context;
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err: anyhow::Error = anyhow::Error::new(io_err)
        .context("level 1")
        .context("level 2")
        .context("level 3");
    
    let mut count = 0;
    let mut current = Some(err.as_ref() as &dyn std::error::Error);
    while let Some(cause) = current {
        count += 1;
        current = cause.source();
    }
    count
}
