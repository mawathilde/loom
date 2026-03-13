use axum::Router;
use axum::routing::get;
use dotenvy::dotenv;
use log::info;
use std::net::SocketAddr;

mod api;
mod db;

use db::DbState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let connection = &mut db::establish_connection();
    db::run_migrations(connection).expect("Failed to run migrations");

    let pool = db::create_pool().expect("Failed to create database pool");
    let db_state = DbState::new(pool);

    let app = Router::new()
        .route("/health", get(api::health::health))
        .with_state(db_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server is running on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
