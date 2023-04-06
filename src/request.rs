use crate::{Error, Result};
use async_trait::async_trait;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;

#[async_trait]
pub(crate) trait RequestBuilderCustomSend {
    /// Custom send method with error checking
    async fn custom_send(self) -> Result<Response>;

    /// Custom send method with error checking and JSON deserialisation
    async fn custom_send_json<T>(self) -> Result<T>
    where
        T: DeserializeOwned;
}

#[async_trait]
impl RequestBuilderCustomSend for RequestBuilder {
    async fn custom_send(self) -> Result<Response> {
        Ok(check_rate_limit(self.send().await?)?.error_for_status()?)
    }

    async fn custom_send_json<T>(self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        Ok(self.custom_send().await?.json().await?)
    }
}

pub(crate) fn check_rate_limit(response: Response) -> Result<Response> {
    if response.status() == StatusCode::TOO_MANY_REQUESTS {
        Err(Error::RateLimitExceeded(
            response
                .headers()
                .get("X-Ratelimit-Reset")
                .map(|header| {
                    header
                        .to_str()
                        .expect("Corrupted ratelimit header")
                        .parse()
                        .expect("Corrupted ratelimit header")
                })
                .expect("Corrupted ratelimit header"),
        ))
    } else {
        Ok(response)
    }
}
