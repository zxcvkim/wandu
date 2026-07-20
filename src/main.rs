use axum::Router;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use wandu::{AppState, config::Config, database, routes};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let config = Config::load()?;
    let pool = database::connect(&config.database_url).await?;

    let addr = config.addr.clone();

    let state = AppState::new(config.clone(), pool);

    sqlx::migrate!("./migrations").run(&state.pool).await?;

    let app = Router::new()
        .nest("/api", routes::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C handler");
}
