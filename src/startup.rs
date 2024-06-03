use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::routes::*;


pub async fn run(listener: TcpListener) {
    let app = Router::new()
        .route("/health_check",get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::serve(listener, app)
        .await
        .unwrap()
}   