use std::{env, sync::Arc};
use tokio::net::TcpListener;

mod jwt;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = sqlx::SqlitePool::connect("sqlite://data.db?mode=rwc")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let state = Arc::new(models::AppState {
        db: pool,
        // TODO: panic when no jwt secret has passed
        jwt_secret: env::var("JWT_SECRET").unwrap_or(String::from("burgers")),
    });

    let app = routes::with_state(state);

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or(String::from("3000"))
    ))
    .await
    .unwrap();

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
