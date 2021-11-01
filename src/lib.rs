mod api_calls;
mod error;
mod request;
mod structures;

pub use api_calls::{mod_calls::*, tag_calls::*, user_calls::*, user_calls::*, version_calls::*};
pub use error::*;
pub use structures::{mod_structs::*, user_structs::*, version_structs::*, Datetime, ID};

pub struct ModrinthAPI {
    client: reqwest::Client,
    user_agent: String,
}

impl ModrinthAPI {
    /// Create a new API instance. `user_agent` should be the name of the program
    pub fn new(user_agent: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            user_agent: user_agent.into(),
        }
    }
}
