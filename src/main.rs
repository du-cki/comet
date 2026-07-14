use std::sync::Arc;
use tokio::net::TcpListener;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = sqlx::SqlitePool::connect("sqlite://data.db?mode=rwc")
        .await
        .unwrap();

    let schema = include_str!("../schema.sql");
    sqlx::query(schema).execute(&pool).await.unwrap();

    let app = routes::new(Arc::new(pool));

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        std::env::var("port").unwrap_or(String::from("3000"))
    ))
    .await
    .unwrap();

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
