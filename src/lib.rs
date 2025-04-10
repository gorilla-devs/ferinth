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

use reqwest::{
    header::{HeaderMap, HeaderValue, InvalidHeaderValue},
    Client,
};
use std::{marker::PhantomData, sync::LazyLock};
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
#[error(transparent)]
pub enum Error {
    #[error("Invalid Modrinth ID or slug")]
    InvalidIDorSlug,
    #[error("Invalid SHA1 hash")]
    InvalidSHA1,
    #[error("You have been rate limited, please wait for {0} seconds")]
    RateLimitExceeded(usize),
    #[error("The API at {} is deprecated", *API_BASE_URL)]
    ApiDeprecated,
    ReqwestError(#[from] reqwest::Error),
    JSONError(#[from] serde_json::Error),
    InvalidHeaderValue(#[from] InvalidHeaderValue),
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
pub struct Ferinth<Auth> {
    client: Client,
    auth: PhantomData<Auth>,
}
pub struct Authenticated;

impl Default for Ferinth<()> {
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
            auth: PhantomData,
        }
    }
}

impl<T> Ferinth<T> {
    fn client_builder(
        name: &str,
        version: Option<&str>,
        contact: Option<&str>,
    ) -> reqwest::ClientBuilder {
        Client::builder().user_agent(format!(
            "{}{}{}",
            name,
            version.map_or("".into(), |version| format!("/{}", version)),
            contact.map_or("".into(), |contact| format!(" ({})", contact))
        ))
    }
}

impl Ferinth<()> {
    /**
    Instantiate the container with the provided
    [user agent](https://docs.modrinth.com/api-spec/#section/User-Agents) details.

    The program `name` is required; `version` and `contact` are optional but recommended.
    */
    pub fn new(name: &str, version: Option<&str>, contact: Option<&str>) -> Self {
        Self {
            auth: PhantomData,
            client: Self::client_builder(name, version, contact)
                .build()
                .expect("Failed to initialise TLS backend"),
        }
    }
}

impl Ferinth<Authenticated> {
    /*
    Instantiate the container with the provided
    [user agent](https://docs.modrinth.com/api-spec/#section/User-Agents) details,
    and authentication `token`.

    The program `name` is required; `version` and `contact` are optional but recommended.

    Fails if the provided `token` cannot be converted into a `HeaderValue`.
    */
    pub fn new<V>(
        name: &str,
        version: Option<&str>,
        contact: Option<&str>,
        token: V,
    ) -> Result<Self>
    where
        V: TryInto<HeaderValue>,
        Error: From<V::Error>,
    {
        Ok(Self {
            auth: PhantomData,
            client: Self::client_builder(name, version, contact)
                .default_headers(HeaderMap::from_iter([(
                    reqwest::header::AUTHORIZATION,
                    token.try_into()?,
                )]))
                .build()
                .expect("Failed to initialise TLS backend"),
        })
    }
}
