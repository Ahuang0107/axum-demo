use axum::http::StatusCode;

pub async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    tracing::info!("signal shutdown");
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (StatusCode::NOT_FOUND, format!("No route {}", uri))
}

pub fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
