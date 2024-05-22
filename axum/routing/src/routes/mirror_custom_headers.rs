use axum::http::HeaderMap;

pub async fn mirror_custom_headers(headers: HeaderMap) -> String {
    headers.get("x-message")
        .and_then(|value| value.to_str().ok())
        .map(|s| s.to_owned())
        .unwrap_or("unknown".to_owned())
}