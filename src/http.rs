use reqwest::{Client, Response};
use std::io::{Error, ErrorKind};

pub async fn post(url: &str, body: String, api_key: &str) -> Result<reqwest::Response, reqwest::Error> {
    let response = Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await;
    response
}

pub async fn get(url: &str, api_key: &str) -> Result<reqwest::Response, reqwest::Error> {
    let response = Client::new()
    .get(url)
    .header("Authorization", format!("Bearer {}", api_key))
    .header("Content-Type", "application/json")
    .send()
    .await;
    response
}

pub async fn process_response(response: Result<Response, reqwest::Error>) -> Result<String, Error> {
    let response = response.map_err(|_| Error::new(ErrorKind::Other, "Request error"))?;
    if response.status().is_success() {
        response.text().await.map_err(|_| Error::new(ErrorKind::Other, "Failed to read response text"))
    } else {
        Err(Error::new(ErrorKind::Other, "Request failed"))
    }
}