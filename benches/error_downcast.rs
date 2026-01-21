use anyhow::{Context, Error};
use std::io;
use thiserror::Error as ThisError;

fn main() {
    divan::main();
}

// Define a custom error type
#[derive(Debug, ThisError)]
#[error("Custom error: {message}")]
struct CustomError {
    message: String,
}

// Helper to create errors
fn make_io_error() -> Error {
    io::Error::new(io::ErrorKind::NotFound, "file not found").into()
}

fn make_error_with_context() -> Error {
    let err: Error = make_io_error();
    match Err::<(), _>(err).context("First context") {
        Err(e) => match Err::<(), _>(e).context("Second context") {
            Err(e2) => e2,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

// Benchmark downcasting
#[divan::bench]
fn downcast_ref_success() {
    let err = make_io_error();
    divan::black_box(err.downcast_ref::<io::Error>());
}

#[divan::bench]
fn downcast_ref_failure() {
    let err = make_io_error();
    divan::black_box(err.downcast_ref::<CustomError>());
}

#[divan::bench]
fn downcast_mut_success() {
    let mut err = make_io_error();
    divan::black_box(err.downcast_mut::<io::Error>());
}

#[divan::bench]
fn downcast_value_success() {
    let err = make_io_error();
    let _ = divan::black_box(err.downcast::<io::Error>());
}

// Benchmark downcasting through context layers
#[divan::bench]
fn downcast_through_context() {
    let err = make_error_with_context();
    divan::black_box(err.downcast_ref::<io::Error>());
}

// Benchmark chain iteration
#[divan::bench]
fn iterate_error_chain() {
    let err = make_error_with_context();
    
    for cause in err.chain() {
        divan::black_box(cause);
    }
}

// Benchmark root_cause access
#[divan::bench]
fn access_root_cause() {
    let err = make_error_with_context();
    divan::black_box(err.root_cause());
}

// Benchmark is test
#[divan::bench]
fn check_is_type() {
    let err = make_io_error();
    divan::black_box(err.is::<io::Error>());
    divan::black_box(err.is::<CustomError>());
}
