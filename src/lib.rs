//! # Ferinth
//!
//! Ferinth is a simple library for using the [Modrinth API](https://github.com/modrinth/labrinth/wiki/API-Documentation) in Rust projects.
//! It uses [Reqwest](https://docs.rs/reqwest/) as its HTTPS client and deserialises responses to strongly typed structs using [SerDe](https://serde.rs/).
//!
//! ## Features
//!
//! This crate includes the following:
//!
//! - All structure definitions based on <https://docs.modrinth.com/api-spec/>
//! - All of the GET calls
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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("A given string was not base62 compliant")]
    NotBase62,
    #[error("A given string was not SHA1 compliant")]
    NotSHA1,
    #[error("You have been rate limited. Please wait for {} seconds", .0)]
    RateLimitExceeded(usize),
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", .0)]
    URLParseError(#[from] url::ParseError),
    #[error("{}", .0)]
    JsonError(#[from] serde_json::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An instance of the API to invoke API calls on.
///
/// To initialise this container,
/// ```rust
/// # use ferinth::Ferinth;
/// # #[tokio::main]
/// # async fn main() -> Result<(), ferinth::Error> {
/// let modrinth = Ferinth::new();
/// // Use the instance to call the API
/// let sodium_mod = modrinth.get_project("sodium").await?;
/// # Ok(()) }
/// ```
#[derive(Debug, Clone)]
pub struct Ferinth {
    client: reqwest::Client,
}

impl Ferinth {
    /// Create a new API instance
    ///
    /// `user_agent` should be the name of the program
    ///
    /// ```rust
    /// # use ferinth::Ferinth;
    /// let modrinth = Ferinth::new();
    /// ```
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}
