use crate::Result;
use reqwest::{RequestBuilder, Response, StatusCode};
use serde::de::DeserializeOwned;

pub(crate) trait RequestBuilderCustomSend {
    /// Build and send `self`, and return the response
    async fn custom_send(self) -> Result<Response>;

    /// Build and send `self`, and deserialise the response to `T` and return it
    async fn custom_send_json<T: DeserializeOwned>(self) -> Result<T>;
}

impl RequestBuilderCustomSend for RequestBuilder {
    async fn custom_send(self) -> Result<Response> {
        Ok(check_rate_limit(self.send().await?)?.error_for_status()?)
    }

    async fn custom_send_json<T: DeserializeOwned>(self) -> Result<T> {
        Ok(self.custom_send().await?.json().await?)
    }
}

fn check_rate_limit(response: Response) -> Result<Response> {
    if response.status() == StatusCode::GONE {
        Err(crate::Error::ApiDeprecated)
    } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
        Err(crate::Error::RateLimitExceeded(
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
