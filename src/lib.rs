/*!
# Ferinth

Ferinth provides Rust API bindings for the [Modrinth API](https://docs.modrinth.com)

## Missing Features

- Search functionality
- Requests that require large body data
- Better organisation of API calls

## Versioning

The major version of this crate's version directly corresponds to the Modrinth API version it uses.
If you want to use the Modrinth API version 2, which is the latest one currently, specify this crate's major version as `2`.

Due to this feature, there will be breaking changes in minor version bumps too!
*/

mod api_calls;
mod request;
pub mod structures;
mod url_ext;

pub use api_calls::{check_id_slug, check_sha1_hash};

use reqwest::{header, Client};
use std::sync::LazyLock;
use url::Url;

/// The base URL for the Modrinth API
pub static BASE_URL: LazyLock<Url> =
    LazyLock::new(|| Url::parse("https://api.modrinth.com/").expect("Invalid base URL"));

/// The base URL for the current version of the Modrinth API
pub static API_BASE_URL: LazyLock<Url> = LazyLock::new(|| {
    BASE_URL
        .join(concat!('v', env!("CARGO_PKG_VERSION_MAJOR"), '/'))
        .expect("Invalid API base URL")
});

#[derive(thiserror::Error, Debug)]
#[error("{}", .0)]
pub enum Error {
    #[error("Invalid Modrinth ID or slug")]
    InvalidIDorSlug,
    #[error("Invalid SHA1 hash")]
    InvalidSHA1,
    #[error("You have been rate limited, please wait for {} seconds", .0)]
    RateLimitExceeded(usize),
    ReqwestError(#[from] reqwest::Error),
    JSONError(#[from] serde_json::Error),
    InvalidHeaderValue(#[from] header::InvalidHeaderValue),
}
pub type Result<T> = std::result::Result<T, Error>;

/**
An instance of the API to invoke API calls on

There are two methods initialise this container:

Use the `Default` implementation to set the user agent based on the crate name and version.
This container will not have authentication.

```ignore
let modrinth = ferinth::Ferinth::default();
```

Use the `new()` function to set a custom user agent and authentication token.

```ignore
let modrinth = ferinth::Ferinth::new(
    env!("CARGO_CRATE_NAME"),
    Some(env!("CARGO_PKG_VERSION")),
    Some("contact@program.com"),
    args.modrinth_token.as_ref(),
)?;
```
*/
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
    /**
    Instantiate the container with the provided [user agent](https://docs.modrinth.com/api-spec/#section/User-Agents) details,
    and an optional GitHub token for `authorisation`.

    The program `name` is required, while `version` and `contact` are optional but recommended.

    This function fails if the GitHub `authorisation` token provided is invalid header data.
    */
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
}
