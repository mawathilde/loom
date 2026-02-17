use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use log::info;
use std::net::SocketAddr;

mod api;

#[tokio::main]
async fn main() {
    dotenv().ok();

    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app = Router::new().route("/health", get(api::health::health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server is running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
