mod db;
mod history;

use std::{env, time::Duration};

use axum::{routing::get, Router};
use db::ClickhouseDatabase;
use tokio::{net::TcpListener, signal};
use tower_http::{
    cors::{self, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[derive(Clone)]
pub struct AppState {
    db: ClickhouseDatabase,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_logger();

    let state = AppState {
        db: ClickhouseDatabase::default(),
    };

    let app = Router::new()
        .route("/history", get(history::history_by_name))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::new(Duration::from_secs(10)),
            CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_methods(cors::Any),
        ))
        .with_state(state);

    let port = env::var("PORT")
        .unwrap_or("3000".to_owned())
        .parse::<u16>()
        .expect("PORT must be a valid 16bit port number");
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;

    tracing::info!("server finished initializing, starting up!");
    axum::serve(listener, app)
        .with_graceful_shutdown(handle_shutdown())
        .await?;

    Ok(())
}

async fn handle_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to intall Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

fn setup_logger() {
    let logger = tracing_subscriber::fmt::layer().json();
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .expect("failed to create logger");
    let exporter = Registry::default().with(logger).with(env_filter);

    tracing::subscriber::set_global_default(exporter).expect("failed to set log exporter");
}
