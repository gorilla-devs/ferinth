use crate::{ModrinthAPI, Result};
use reqwest::{header::USER_AGENT, Response};

pub(crate) const API_URL_BASE: &str = "https://api.modrinth.com/api/v1";

pub(crate) async fn request_rel(client: &ModrinthAPI, route: String) -> Result<Response> {
    Ok(request(client, format!("{}{}", API_URL_BASE, route)).await?)
}

pub(crate) async fn request(client: &ModrinthAPI, url: String) -> Result<Response> {
    let request = client
        .client
        .get(url)
        .header(USER_AGENT, &client.user_agent);

    let response = request.send().await?.error_for_status()?;

    Ok(response)
}
