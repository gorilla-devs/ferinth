//! # Ferinth
//!
//! Ferinth is a simple library for using the [Modrinth API](https://github.com/modrinth/labrinth/wiki/API-Documentation) in Rust projects.
//! It uses [Reqwest](https://docs.rs/reqwest/) as its HTTPS client and deserialises responses to strongly typed structs using [SerDe](https://serde.rs/).
//!
//! ## Features
//!
//! This crate includes the following:
//!
//! - All structure definitions based on <https://github.com/modrinth/labrinth/wiki/API-Documentation#structure-definitions>
//! - Includes the following API calls:
//!   - Get project by project ID
//!   - List project categories
//!   - List mod loaders
//!   - List game versions
//!   - Get user by user ID
//!   - List team members by team ID
//!   - Get version by version ID
//!   - Get version by file hash
//!   - List versions by project ID
//!   - Download version file
//!
//! URL traversal is blocked because all IDs are verified.
//! ```
//! # let modrinth = ferinth::Ferinth::new("ferinth-example");
//! # tokio_test::block_on( async {
//! assert!(modrinth.get_project("sodium/version").await.is_err());
//! # } );
//! ```
//!
//! This crate uses [Rustls](https://docs.rs/rustls/) rather than OpenSSL, because OpenSSL is outdated and slower.
//!
//! The following features have not yet been implemented
//! - Search projects
//! - User authentication
//! - Get current user (constrained by the lack of user authentication)
//!
//! Unfortunately, I am not planning to implement any of these features in this first version due to poor documentation.
//! I will add these features in the next version, version 2, once the [Modrinth API v2](https://docs.modrinth.com/api-spec/) rolls out.
//!
//! ## Example (and Tutorial)
//!
//! The full source code for this can be found as a binary project at [example/](https://github.com/theRookieCoder/ferinth/tree/master/example).
//!
//! In this example, we are going to download and install the latest version of [Sodium](https://modrinth.com/mod/sodium).
//!
//! ```rust
//! # use ferinth::Ferinth;
//! # use std::{fs::File, io::Write};
//! # use thiserror::Error;
//! #
//! # #[derive(Debug, Error)]
//! # enum Error {
//! #     #[error("API error occured")]
//! #     FerinthError(#[from] ferinth::Error),
//! #     #[error("IO error occured")]
//! #     IOError(#[from] std::io::Error),
//! # }
//! # #[tokio::main]
//! # async fn main() -> Result<(), Error> {
//! // First, let's initialise the API
//! // You should replace 'example' with your application's name
//! let api = Ferinth::new("example");
//!
//! // Now, lets get the Sodium mod
//! // You can use the project ID, or the project slug
//! // The project ID will never change but the project slug can change at anytime
//! // Using the project slug
//! let sodium = api.get_project("sodium").await?;
//! // Using the project ID
//! let sodium = api.get_project("AANobbMI").await?;
//!
//! // Now lets get the versions that the Sodium mod has
//! let sodium_versions = api.list_versions("AANobbMI").await?;
//!
//! // The versions are sorted chronologically so the first element should be the latest one
//! let latest_version = &sodium_versions[0];
//! // And now we can get this version's mod file, which is called a version file
//! let version_file = &latest_version.files[0];
//!
//! // Then we can download this version file
//! let contents = api.download_version_file(version_file).await?;
//! // And next, lets open the file we want to write this to
//! let mut mod_file = File::create("Sodium.jar")?;
//! // And finally, we can write the contents to mod_file
//! mod_file.write_all(&contents)?;
//!
//! // Now you can use load the JAR file using a mod loader. To play Sodium. you should use Fabric
//! # Ok(())
//! # }
//! ```

mod api_calls;
mod request;
pub mod structures;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("A given string was not base62 compliant")]
    NotBase62,
    #[error("A given string was not SHA1 compliant")]
    NotSHA1,
    #[error("{}", .0)]
    ReqwestError(#[from] reqwest::Error),
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An instance of the API to invoke API calls on.
///
/// To initialise this container,
/// ```rust
/// # use ferinth::Ferinth;
/// let modrinth = Ferinth::new("ferinth-example");
/// // Use the instance to call the API
/// let sodium_mod = modrinth.get_project("sodium");
/// ```
pub struct Ferinth {
    client: reqwest::Client,
    user_agent: String,
}

impl Ferinth {
    /// Create a new API instance
    ///
    /// `user_agent` should be the name of the program
    ///
    /// ```rust
    /// # use ferinth::Ferinth;
    /// let modrinth = Ferinth::new("ferinth-example");
    /// ```
    pub fn new(user_agent: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            user_agent: user_agent.into(),
        }
    }
}
