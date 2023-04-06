//! # Ferinth
//!
//! [![github badge](https://img.shields.io/badge/GitHub-Ferinth-informational?style=for-the-badge&logo=github&labelColor=555555)](https://github.com/gorilla-devs/ferinth)
//! [![crates badge](https://img.shields.io/crates/v/ferinth?logo=rust&style=for-the-badge)](https://crates.io/crates/ferinth)
//! [![docs.rs](https://img.shields.io/docsrs/ferinth?logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K&style=for-the-badge)](https://docs.rs/ferinth)
//!
//! Ferinth is a simple library to use the Modrinth API in Rust projects
//!
//! It provides API bindings for the [Modrinth API](https://docs.modrinth.com), is intuitive to use, and provides typed structs for all the structures used.
//!
//! ## Use
//!
//! **The major version of this crate's version directly corresponds to the Modrinth API version it uses**.
//!
//! So for example if you want to use the Modrinth API version 2, which is the latest one available now, then specify this crate's major version as `2`.

#![deny(clippy::unwrap_used)]
#![warn(missing_docs)]

mod api_calls;
mod request;
pub mod structures;
mod url_ext;

use once_cell::sync::Lazy;
use reqwest::{header, Client};
use url::Url;

/// The base URL for the Modrinth API
pub static BASE_URL: Lazy<Url> =
    Lazy::new(|| Url::parse("https://api.modrinth.com/").expect("Invalid base URL"));

/// The base URL for the current version of the Modrinth API
pub static API_BASE_URL: Lazy<Url> = Lazy::new(|| {
    BASE_URL
        .join(concat!(
            'v',
            env!(
                "CARGO_PKG_VERSION_MAJOR",
                "Make sure you're compiling with cargo!",
            ),
            '/'
        ))
        .expect("Invalid API base URL")
});

#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum Error {
    #[error("Ivalid Modrinth ID or slug")]
    InvalidIDorSlug,
    #[error("Invalid SHA1 hash")]
    InvalidSHA1,
    #[error("You have been rate limited, please wait for {} seconds", .0)]
    RateLimitExceeded(usize),
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{}", .0)]
    JSONError(#[from] serde_json::Error),
    #[error("The GitHub token provided is invalid")]
    InvalidGitHubToken(#[from] header::InvalidHeaderValue),
}

#[allow(missing_docs)]
pub type Result<T> = std::result::Result<T, Error>;

/// An instance of the API to invoke API calls on.
///
/// There are two methods initialise this container:
///
/// Use the `Default` implementation to set the user agent based on the crate name and version.
///    This will not have authentication.
///
/// ```ignore
/// let modrinth = ferinth::Ferinth::default();
/// ```
///
/// Use the `new()` function to set a custom user agent and authentication token.
///
/// ```ignore
/// let modrinth = ferinth::Ferinth::new(
///     env!("CARGO_CRATE_NAME"),
///     Some(env!("CARGO_PKG_VERSION")),
///     Some("contact@program.com"),
///     args.modrinth_token.as_ref(),
/// )?;
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
                .expect("Failed to initialise TLS backend"),
        }
    }
}

impl Ferinth {
    /// Instantiate the container with the provided [user agent](https://docs.modrinth.com/api-spec/#section/User-Agents) details,
    /// and an optional GitHub token for `authorisation`.
    ///
    /// `program_name` is required, while `version` and `contact` are optional but recommended.
    ///
    /// This function fails if the GitHub `authorisation` token provided is invalid header data.
    pub fn new(
        name: &str,
        version: Option<&str>,
        contact: Option<&str>,
        authorisation: Option<&str>,
    ) -> Result<Self> {
        use header::{HeaderMap, HeaderValue};

        Ok(Self {
            client: Client::builder()
                .user_agent(format!(
                    "{}{}{}",
                    name,
                    version.map_or("".into(), |version| format!("/{}", version)),
                    contact.map_or("".into(), |contact| format!(" ({})", contact))
                ))
                .default_headers(if let Some(authorisation) = authorisation {
                    HeaderMap::from_iter(vec![(
                        header::AUTHORIZATION,
                        HeaderValue::from_str(authorisation)?,
                    )])
                } else {
                    HeaderMap::new()
                })
                .build()
                .expect("Failed to initialise TLS backend"),
        })
    }

    /// Check `self`'s connection to the Modrinth API.
    ///
    /// ```rust
    /// # #[tokio::main]
    /// # async fn main() -> ferinth::Result<()> {
    /// # let modrinth = ferinth::Ferinth::default();
    /// modrinth.check_api().await
    /// # }
    /// ```
    pub async fn check_api(&self) -> Result<()> {
        use request::RequestBuilderCustomSend;

        self.client.get(BASE_URL.as_ref()).custom_send().await?;
        Ok(())
    }
}
