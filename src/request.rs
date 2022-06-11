use crate::{Error, Ferinth, Result};
use reqwest::{IntoUrl, Response, StatusCode, Url};
use serde::Serialize;

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.modrinth.com/v2/").unwrap();
}

impl Ferinth {
    /// Perform a GET request on `url`
    pub(crate) async fn get(&self, url: impl IntoUrl + Clone) -> Result<Response> {
        let response = self.client.get(url.clone()).send().await?;
        if StatusCode::TOO_MANY_REQUESTS == response.status() {
            Err(Error::RateLimitExceeded(
                response
                    .headers()
                    .get("X-Ratelimit-Reset")
                    .map(|header| header.to_str().unwrap().parse().unwrap())
                    .unwrap(),
            ))
        } else {
            Ok(response.error_for_status()?)
        }
    }

    /// Perform a POST request on `url` with body
    pub(crate) async fn post(
        &self,
        url: impl IntoUrl + Clone,
        body: impl Serialize,
    ) -> Result<Response> {
        let response = self.client.post(url.clone()).json(&body).send().await?;
        if StatusCode::TOO_MANY_REQUESTS == response.status() {
            Err(Error::RateLimitExceeded(
                response
                    .headers()
                    .get("X-Ratelimit-Reset")
                    .map(|header| header.to_str().unwrap().parse().unwrap())
                    .unwrap(),
            ))
        } else {
            Ok(response.error_for_status()?)
        }
    }
}
