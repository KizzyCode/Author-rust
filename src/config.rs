//! The server config

use crate::err;
use crate::error::Error;
use std::borrow::Cow;
use std::env::{self, VarError};
use std::net::SocketAddr;

/// The server config
#[derive(Debug, Clone)]
#[allow(non_snake_case, reason = "We want to map the exact naming of the environment variables")]
pub struct Config {
    /// The socket address to listen on
    ///
    /// # Example
    /// An `address:port` combination; defaults to [`Self::AUTHOR_LISTEN_DEFAULT`].
    pub AUTHOR_LISTEN: SocketAddr,
    /// The maximum amount of open connections
    ///
    /// # Discussion
    /// Each opened connection requires at least one separate thread; depending on your OS and environment this may
    /// cause significant load. defaults to [`Self::AUTHOR_CONNMAX_DEFAULT`].
    pub AUTHOR_CONNMAX: usize,
    /// The authentication response header to set on success
    ///
    /// # Example
    /// A HTTP header name; defaults to [`Self::AUTHOR_HEADER_DEFAULT`]
    pub AUTHOR_HEADER: Cow<'static, str>,
}
impl Config {
    /// The default listening address if [`Self::AUTHOR_LISTEN`] is not speficied
    const AUTHOR_LISTEN_DEFAULT: &str = "[::]:8080";
    /// The default connection limit if [`Self::AUTHOR_CONNMAX`] is not speficied
    const AUTHOR_CONNMAX_DEFAULT: &str = "1024";
    /// The default response header if [`Self::AUTHOR_HEADER`] is not speficied
    const AUTHOR_HEADER_DEFAULT: &str = "X-Forward-AuthorAuthId";

    /// Gets the config from the environment
    pub fn from_env() -> Result<Self, Error> {
        // Load config
        Ok(Config {
            AUTHOR_LISTEN: Self::author_listen()?,
            AUTHOR_CONNMAX: Self::author_connmax()?,
            AUTHOR_HEADER: Self::author_header()?,
        })
    }

    /// Parses the `AUTHOR_LISTEN` environment variable, or falls back to [`Self::AUTHOR_LISTEN_DEFAULT`]
    fn author_listen() -> Result<SocketAddr, Error> {
        let address = Self::env("AUTHOR_LISTEN", Some(Self::AUTHOR_LISTEN_DEFAULT))?;
        Ok(address.parse()?)
    }

    /// Parses the `AUTHOR_CONNMAX` environment variable, or falls back to [`Self::AUTHOR_CONNMAX_DEFAULT`]
    fn author_connmax() -> Result<usize, Error> {
        let maxconn = Self::env("AUTHOR_CONNMAX", Some(Self::AUTHOR_CONNMAX_DEFAULT))?;
        Ok(maxconn.parse()?)
    }

    /// Parses the `AUTHOR_HEADER` environment variable, or falls back to [`Self::AUTHOR_HEADER_DEFAULT`]
    fn author_header() -> Result<Cow<'static, str>, Error> {
        let header = Self::env("AUTHOR_HEADER", Some(Self::AUTHOR_HEADER_DEFAULT))?;
        Ok(header)
    }

    /// Gets the environment variable with the given name or returns the default value
    fn env(name: &str, default: Option<&'static str>) -> Result<Cow<'static, str>, Error> {
        match (env::var(name), default) {
            (Ok(value), _) => Ok(Cow::Owned(value)),
            (Err(VarError::NotPresent), Some(default)) => Ok(Cow::Borrowed(default)),
            (Err(VarError::NotPresent), _) => Err(err!(r#"Missing environment variable "{name}""#)),
            (Err(e), _) => Err(err!(with: e, r#"Invalid environment variable "{name}""#)),
        }
    }
}
