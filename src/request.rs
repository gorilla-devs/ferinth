use crate::{Ferinth, Result};
use reqwest::{header::USER_AGENT, IntoUrl, Response};

const API_URL_BASE: &str = "https://api.modrinth.com/api/v1";

/// Perform a GET request on base url + `route` using `client`
pub async fn request_rel(client: &Ferinth, route: String) -> Result<Response> {
    Ok(request(client, format!("{}{}", API_URL_BASE, route)).await?)
}

pub async fn request(client: &Ferinth, url: impl IntoUrl) -> Result<Response> {
    let request = client
        .client
        .get(url)
        .header(USER_AGENT, &client.user_agent);

    let response = request.send().await?.error_for_status()?;

    Ok(response)
}
