use crate::{Error, Ferinth, Result};
use reqwest::{StatusCode, Url};
use serde::{de::DeserializeOwned, Serialize};

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.modrinth.com/v2/").unwrap();
}

impl Ferinth {
    /// Perform a GET request to `url`, and deserialise the response
    pub(crate) async fn get<T>(&self, url: Url) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self.client.get(url).send().await?;
        if StatusCode::TOO_MANY_REQUESTS == response.status() {
            Err(Error::RateLimitExceeded(
                response
                    .headers()
                    .get("X-Ratelimit-Reset")
                    .map(|header| header.to_str().unwrap().parse().unwrap())
                    .unwrap(),
            ))
        } else {
            Ok(response.error_for_status()?.json().await?)
        }
    }

    /// Perform a GET request to `url` with `query` parameters, and deserialise the response
    pub(crate) async fn get_with_query<T, S1, S2>(
        &self,
        mut url: Url,
        query: &[(S1, S2)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        url.query_pairs_mut().extend_pairs(query);
        self.get(url).await
    }

    /// Perform a POST request to `url` with `body`, and deserialise the response
    pub(crate) async fn post<T, B>(&self, url: Url, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let response = self.client.post(url).json(body).send().await?;
        if StatusCode::TOO_MANY_REQUESTS == response.status() {
            Err(Error::RateLimitExceeded(
                response
                    .headers()
                    .get("X-Ratelimit-Reset")
                    .map(|header| header.to_str().unwrap().parse().unwrap())
                    .unwrap(),
            ))
        } else {
            Ok(response.error_for_status()?.json().await?)
        }
    }

    /// Perform a POST request to `url` with `body` and `query` parameters, and deserialise the response
    pub(crate) async fn post_with_query<T, B, S1, S2>(
        &self,
        mut url: Url,
        body: &B,
        query: &[(S1, S2)],
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize,
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        url.query_pairs_mut().extend_pairs(query);
        self.post(url, body).await
    }
}
