use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use diesel::prelude::*;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncMysqlConnection, RunQueryDsl,
};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod api;
mod schemas;
mod utils;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncMysqlConnection>>;

struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncMysqlConnection>>,
);

#[axum::async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(utils::internal_error)?;

        Ok(Self(conn))
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_demo=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = AsyncDieselConnectionManager::<AsyncMysqlConnection>::new(
        "mysql://root:12345678@localhost:3306/axum",
    );
    let pool = Pool::builder().build(config).await.unwrap();

    let app = Router::new()
        .route("/engage/list", get(list_engages))
        .with_state(pool)
        .fallback(utils::fallback)
        .nest("/api", api::app());

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 27788));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(utils::shutdown_signal())
        .await
        .unwrap();
}

async fn list_engages(
    DatabaseConnection(mut conn): DatabaseConnection,
) -> Result<Json<Vec<schemas::Engage>>, (StatusCode, String)> {
    let res = schemas::engages::table
        .select(schemas::Engage::as_select())
        .load(&mut conn)
        .await
        .map_err(utils::internal_error)?;
    Ok(Json(res))
}
