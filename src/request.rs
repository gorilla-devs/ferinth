use crate::{Error, Ferinth, Result};
use reqwest::{IntoUrl, Response, StatusCode, Url};
lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.modrinth.com/v2/").unwrap();
}

/// Perform a GET request on `url` using the HTTPS client and user agent from `client`
pub(crate) async fn request(client: &Ferinth, url: impl IntoUrl + Clone) -> Result<Response> {
    let response = client.client.get(url.clone()).send().await?;
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
