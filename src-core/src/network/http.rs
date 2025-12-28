use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct FetchOptions {
    pub method: Option<String>,
    pub headers: Option<HashMap<String, String>>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FetchResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// Perform an HTTP request.
pub async fn http_fetch(
    url: String,
    options: Option<FetchOptions>,
) -> Result<FetchResponse, String> {
    let client = reqwest::Client::new();

    let method = options
        .as_ref()
        .and_then(|o| o.method.as_deref())
        .unwrap_or("GET");

    let mut request = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        "PATCH" => client.patch(&url),
        "HEAD" => client.head(&url),
        _ => return Err(format!("Unsupported HTTP method: {}", method)),
    };

    if let Some(ref opts) = options {
        if let Some(ref headers) = opts.headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        if let Some(ref body) = opts.body {
            // Always decode base64 body and send as binary
            let decoded = base64::decode(body)
                .map_err(|e| format!("Failed to decode base64 body: {}", e))?;
            request = request.body(decoded);
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = response.status().as_u16();

    let mut response_headers = HashMap::new();
    for (key, value) in response.headers() {
        if let Ok(value_str) = value.to_str() {
            response_headers.insert(key.to_string(), value_str.to_string());
        }
    }

    // Always return response as base64 to handle arbitrary binary data
    let bytes = response
        .bytes()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    let body = base64::encode(&bytes);

    Ok(FetchResponse {
        status,
        headers: response_headers,
        body,
    })
}
