use crate::{Ferinth, Result};
use reqwest::{IntoUrl, Response, Url};

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.modrinth.com/v2/").unwrap();
}

/// Perform a GET request on `url` using the HTTPS client and user agent from `client`
pub(crate) async fn request(client: &Ferinth, url: impl IntoUrl) -> Result<Response> {
    Ok(client.client.get(url).send().await?.error_for_status()?)
}
