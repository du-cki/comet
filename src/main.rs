use std::{net::SocketAddr, sync::Arc};

use tracing::info;

mod models;
mod routes;
mod settings;
mod utils;

use crate::settings::Settings;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = sqlx::SqlitePool::connect("sqlite://data.db?mode=rwc")
        .await
        .unwrap();

    let schema = include_str!("../schema.sql");
    sqlx::query(schema).execute(&pool).await.unwrap();

    let config = Settings::new().unwrap();
    let app = routes::create(Arc::new(pool), &config);

    let addr = SocketAddr::from((config.bind_addr, config.bind_port));
    info!("listening on http://{}/", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
