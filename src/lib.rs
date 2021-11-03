//! A simple library to use the Modrinth API in Rust projects
//! 
//! Ferinth provides API bindings for the Modrinth API. It is intuitive to use and provides typed structs for all the data used
//! 
//! # Example
//! You can find an example [here](https://github.com/theRookieCoder/ferinth/tree/master/example/)
//! 
//! 

mod api_calls;
mod request;
pub mod structures;

/// A type alias for `reqwest::Error` so that you do not have to specify it in your `Cargo.toml`
pub type Error = reqwest::Error;
pub(crate) type Result<T> = std::result::Result<T, Error>;

/// An instance of the API
/// 
/// Invoke API calls on this container
pub struct Ferinth {
    client: reqwest::Client,
    user_agent: String,
}

impl Ferinth {
    /// Create a new API instance
    /// 
    /// `user_agent` should be the name of the program
    pub fn new(user_agent: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            user_agent: user_agent.into(),
        }
    }
}
