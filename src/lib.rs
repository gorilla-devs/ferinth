//! # Ferinth
//!
//! Ferinth is a simple library for using the [Modrinth API](https://github.com/modrinth/labrinth/wiki/API-Documentation) in Rust.
//! It uses [Reqwest](https://docs.rs/reqwest/) as its HTTPS client and deserialises responses to strongly typed structs using [SerDe](https://serde.rs/).
//!
//! ## Features
//!
//! This crate includes the following:
//!
//! - All structure definitions based on <https://docs.modrinth.com/api-spec/>
//! - All of the GET and POST calls that don't require authentication
//!
//! This crate uses [RusTLS](https://docs.rs/rustls/) rather than OpenSSL, because OpenSSL is outdated and slower.
//!
//! The following features still need to be implemented
//! - Search projects
//! - User authentication
//! - Other types of requests

mod api_calls;
mod request;
pub mod structures;
mod url_join_ext;

use reqwest::Client;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("A given string was not base62 compliant")]
    NotBase62,
    #[error("A given string was not SHA1 compliant")]
    NotSHA1,
    #[error("You have been rate limited, please wait for {} seconds", .0)]
    RateLimitExceeded(usize),
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", .0)]
    JSONError(#[from] serde_json::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An instance of the API to invoke API calls on.
///
/// To initialise this container,
/// ```rust
/// # use ferinth::Ferinth;
/// # #[tokio::main]
/// # async fn main() -> Result<(), ferinth::Error> {
/// let modrinth = Ferinth::default();
/// // Use the instance to call the API
/// let sodium_mod = modrinth.get_project("sodium").await?;
/// # Ok(()) }
/// ```
#[derive(Debug, Clone)]
pub struct Ferinth {
    client: Client,
}

impl Default for Ferinth {
    fn default() -> Self {
        Self {
            client: Client::builder()
                .user_agent(concat!(
                    env!("CARGO_CRATE_NAME"),
                    "/",
                    env!("CARGO_PKG_VERSION")
                ))
                .build()
                .unwrap(),
        }
    }
}

impl Ferinth {
    /// Instantiate the container with the provided [user agent](https://docs.modrinth.com/api-spec/#section/User-Agents).
    /// `program_name` is required, and the `version` and `contact` are optional but recommended by Modrinth.
    pub fn new(program_name: &str, version: Option<&str>, contact: Option<&str>) -> Self {
        Self {
            client: Client::builder()
                .user_agent(format!(
                    "{}{}{}",
                    program_name,
                    version.map_or("".into(), |version| format!("/{}", version)),
                    contact.map_or("".into(), |contact| format!(" ({})", contact))
                ))
                .build()
                .unwrap(),
        }
    }
}
