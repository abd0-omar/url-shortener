use crate::routes::{create_link, get_link_statistics, redirect, update_link};
use axum::routing::post;
use std::error::Error;

use axum::{routing::get, Router};
use axum_prometheus::PrometheusMetricLayer;
use routes::heatlh;
use sqlx::postgres::PgPoolOptions;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "link_shortener=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // I could use the config way like zero2prod
    let db_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL is a required enviroment variable, that small file ends with.env");

    let db = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    let (prometheus_layer, metric_handler) = PrometheusMetricLayer::pair();

    let app = Router::new()
        .route("/create", post(create_link))
        .route("/:id/statistics", get(get_link_statistics))
        .route("/:id", get(redirect).patch(update_link))
        .route("/health", get(heatlh))
        .route("/metrics", get(|| async move { metric_handler.render() }))
        .layer(TraceLayer::new_for_http())
        .layer(prometheus_layer)
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("could not \"listen\", like Van Gogh");

    tracing::debug!(
        "listening live on {}",
        listener
            .local_addr()
            .expect("Could not convert listener address to local address")
    );

    axum::serve(listener, app)
        .await
        .expect("Server created unsuccessfully");
    Ok(())
}
