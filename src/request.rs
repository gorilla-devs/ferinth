use crate::{Ferinth, Result};
use reqwest::{header::USER_AGENT, IntoUrl, Response};

const API_URL_BASE: &str = "https://api.modrinth.com/v2";

/// Perform a GET request on base url + `route` using `client`
pub(crate) async fn request_rel(client: &Ferinth, route: String) -> Result<Response> {
    request(client, format!("{}{}", API_URL_BASE, route)).await
}

/// Perform a GET request on `url` using HTTP(S) client and user agent, `client`
pub(crate) async fn request(client: &Ferinth, url: impl IntoUrl) -> Result<Response> {
    let request = client
        .client
        .get(url)
        .header(USER_AGENT, &client.user_agent);

    Ok(request.send().await?.error_for_status()?)
}
