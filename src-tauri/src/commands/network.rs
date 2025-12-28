use tauri::command;
use wealthfolio_core::network::{http_fetch as core_http_fetch, FetchOptions, FetchResponse};

#[command]
pub async fn http_fetch(
    url: String,
    options: Option<FetchOptions>,
) -> Result<FetchResponse, String> {
    core_http_fetch(url, options).await
}
