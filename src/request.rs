use crate::{Error, Ferinth, Result};
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use url::Url;

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

impl Ferinth {
    /// Perform a GET request to `url`, and deserialise the response
    pub(crate) async fn get<T>(&self, url: Url) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        Ok(check_rate_limit(response)?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Perform a POST request to `url` with `body` of `content_type`, and deserialise the response
    pub(crate) async fn post<B: Into<Body>>(
        &self,
        url: Url,
        body: B,
        content_type: &str,
    ) -> Result<Response> {
        let response = self
            .client
            .post(url)
            .body(body)
            .header(CONTENT_TYPE, HeaderValue::from_str(content_type)?)
            .send()
            .await?;
        Ok(check_rate_limit(response)?.error_for_status()?)
    }

    /// Perform a POST request to `url` with `body`, and deserialise the response
    pub(crate) async fn post_json<T, B>(&self, url: Url, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let response = self.client.post(url).json(body).send().await?;
        Ok(check_rate_limit(response)?
            .error_for_status()?
            .json()
            .await?)
    }

    /// Perform a DELETE request to `url`
    pub(crate) async fn delete(&self, url: Url) -> Result<Response> {
        let response = self.client.delete(url).send().await?;
        Ok(check_rate_limit(response)?.error_for_status()?)
    }
}
