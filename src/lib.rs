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

mod config;
mod error;
mod utils;
mod validators;

pub use crate::config::Config;
use crate::error::Error;
use crate::utils::errorresponse::ErrorResponse;
use crate::validators::traefik;
use ehttpd::Server;
use ehttpd::http::Response;
use std::convert::Infallible;

/// The server runloop
pub fn serve(config: Config) -> Result<Infallible, Error> {
    // Start server
    let server_listen = config.AUTHOR_LISTEN;
    let server = Server::with_request_response(config.AUTHOR_CONNMAX, move |request| {
        // Split request path for route matching
        let target = request.target.clone();
        let mut target = target.split(|byte| *byte == b'/');

        // Match path components
        let method = request.method.as_ref();
        match [target.next(), target.next(), target.next(), target.next(), target.next()] {
            // `/traefik/xforwardedtlsclientcertinfo/{regex}`: Matches an mTLS cert info against a regex
            [Some(b""), Some(b"traefik"), Some(b"xforwardedtlsclientcertinfo"), Some(regex), None] => match method {
                b"GET" | b"HEAD" => traefik::xforwardedtlsclientcertinfo(&config, request, regex),
                _ => Response::new_405_methodnotallowed(),
            },
            // Route did not match
            _ => {
                let message = request.target.iter().chain(b" not found").copied();
                ErrorResponse::new_404_notfound(message.collect::<Vec<u8>>())
            }
        }
    });

    // Start the server and dispatch connections
    let Err(e) = server.accept(server_listen);
    Err(err!(with: e, "server task failed"))
}
