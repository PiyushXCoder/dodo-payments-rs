use std::sync::Arc;

use reqwest::Response;
use serde::{Deserialize, Serialize, de};

use crate::{client::Handle, errors::Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

pub fn parse_response<'a, T: de::Deserialize<'a>>(body: &'a str) -> Result<T, Error> {
    let response = serde_json::from_str::<T>(body);
    match response {
        Ok(v) => Ok(v),
        Err(e) => {
            let error_response = serde_json::from_str::<ErrorResponse>(body);
            match error_response {
                Ok(err_resp) => Err(Error::ErrorResponse(err_resp)),
                Err(_) => Err(Error::SerdeJson(e)),
            }
        }
    }
}

pub async fn make_reqwest(
    handle: Arc<Handle>,
    method: reqwest::Method,
    url: &str,
    body: Option<String>,
) -> Result<Response, Error> {
    let url = format!("{}{}", handle.config.environment.base_url(), url);
    let response = reqwest::Client::new()
        .request(method, url)
        .header(
            "Authorization",
            format!("Bearer {}", handle.config.bearer_token.as_str()),
        )
        .header("Content-Type", "application/json")
        .body(body.unwrap_or_default())
        .send()
        .await?;
    Ok(response)
}
