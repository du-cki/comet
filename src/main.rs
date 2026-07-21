use std::{env, sync::Arc};
use tokio::{net::TcpListener, sync::broadcast};

mod exif;
mod jwt;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let pool = sqlx::SqlitePool::connect("sqlite://data.db?mode=rwc")
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let (tx, _) = broadcast::channel(100);

    let state = Arc::new(models::AppState {
        db: pool,
        // TODO: panic when no jwt secret has passed
        jwt_secret: env::var("JWT_SECRET").unwrap_or(String::from("burgers")),
        config: models::Config {
            // TODO: configure for docker env:
            file_save_path: String::from("uploads/"),

            // TODO: move to settings table:
            file_name_length: 8,
            enforce_file_extensions: true,
        },
        tx,
    });

    let mut app = routes::with_state(state);

    #[cfg(debug_assertions)]
    {
        use tower_http::cors::{Any, CorsLayer};

        tracing::warn!("running in debug mode, CORS is disabled");
        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .expose_headers(Any)
            .allow_methods(Any);

        app = app.layer(cors);
    }

    let listener = TcpListener::bind(format!(
        "0.0.0.0:{}",
        env::var("PORT").unwrap_or(String::from("3000"))
    ))
    .await
    .unwrap();

    tracing::info!("listening on http://{}", listener.local_addr().unwrap());
    let _ = axum::serve(listener, app).await;
}
