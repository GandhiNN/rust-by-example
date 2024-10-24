use std::collections::HashMap;

use reqwest::{header, Error};

pub async fn headers_for_requests() -> Result<(), Error> {
    // Set up the URL and headers for the request
    let url = "https://example.com/api";
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("reqwest"),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    // Set up the query parameters for the request
    let mut params = HashMap::new();
    params.insert("foo", "bar");
    params.insert("baz", "qux");

    // Make the request
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    // Handle the response
    println!("{:?}", response);

    Ok(())
}

pub async fn handle_error() -> Result<(), Error> {
    let response = reqwest::get("https://www.example.com").await?;

    match response.status().as_u16() {
        200..=299 => {
            let body = response.text().await?;
            println!("Success! Body:\n{}", body);
        }
        400..=599 => {
            let status = response.status();
            let error_message = response.text().await?;
            println!("Error {}: {}", status, error_message);
        }
        _ => {
            println!("Unexpected status code: {}", response.status());
        }
    }
    Ok(())
}

pub async fn http_get_request() -> Result<(), Error> {
    let response = reqwest::get("https://www.example.com").await?;
    println!("Status: {}", response.status());

    let body = response.text().await?;
    println!("Body: {}", body);

    Ok(())
}

pub async fn http_post_request() -> Result<(), Error> {
    let url = "https://httpbin.org/post";
    let json_data = r#"{"name": "John Doe", "email": "john.doe@example.com"}"#;

    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(json_data.to_owned())
        .send()
        .await?;

    println!("Status: {}", response.status());

    let response_body = response.text().await?;
    println!("Response body: {}", response_body);

    Ok(())
}
