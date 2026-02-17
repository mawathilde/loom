use axum::body::Body;
use axum::response::Response;

pub async fn health() -> Response {
    Response::new(Body::from("OK"))
}
