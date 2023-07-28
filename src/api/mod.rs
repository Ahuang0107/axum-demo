use axum::routing::get;
use axum::Router;

pub fn app() -> Router {
    Router::new().route("/public", get(hello_world))
}

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}
