use std::{fs, net::SocketAddr, sync::Arc};

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

    fs::create_dir_all(&config.file_save_path).unwrap();

    let app = routes::new(Arc::new(pool), &config);

    let addr = SocketAddr::from((config.addr, config.port));
    info!("listening on http://{}/", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
