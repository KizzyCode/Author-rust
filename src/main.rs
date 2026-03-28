#![doc = include_str!("../README.md")]
// Clippy lints
#![warn(clippy::large_stack_arrays)]
#![warn(clippy::arithmetic_side_effects)]
#![warn(clippy::expect_used)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::indexing_slicing)]
#![warn(clippy::panic)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::unreachable)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::allow_attributes_without_reason)]
#![warn(clippy::cognitive_complexity)]
#![forbid(unsafe_code)]

use author::Config;
use std::process;

pub fn main() {
    // Load config and enter server runloop
    let Err(e) = Config::from_env().and_then(|config| {
        // Display user friendly status message and exec server
        eprintln!("Starting author server on {}...", config.AUTHOR_LISTEN);
        author::serve(config)
    });

    // Log error and fail
    e.log_to_stderr();
    process::exit(1);
}
