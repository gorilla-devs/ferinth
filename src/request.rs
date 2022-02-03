use crate::{Ferinth, Result};
use reqwest::{header::USER_AGENT, IntoUrl, Response, Url};

lazy_static::lazy_static! {
    pub(crate) static ref API_URL_BASE: Url = Url::parse("https://api.modrinth.com/v2/").unwrap();
}

/// Perform a GET request on `url` using the HTTPS client and user agent from `client`
pub(crate) async fn request(client: &Ferinth, url: impl IntoUrl) -> Result<Response> {
    let request = client
        .client
        .get(url)
        .header(USER_AGENT, &client.user_agent);

    Ok(request.send().await?.error_for_status()?)
}
