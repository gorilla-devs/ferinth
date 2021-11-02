mod api_calls;
mod request;
pub mod structures;

pub(crate) type Result<T> = std::result::Result<T, reqwest::Error>;

pub struct Ferinth {
    client: reqwest::Client,
    user_agent: String,
}

impl Ferinth {
    /// Create a new API instance. `user_agent` should be the name of the program
    pub fn new(user_agent: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            user_agent: user_agent.into(),
        }
    }
}
