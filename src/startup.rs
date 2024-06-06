use axum::Router;
use axum::routing::{get, post};
use tokio::net::TcpListener;
use crate::routes::*;
use sqlx::{PgConnection};
use std::sync::Arc;


pub async fn run(listener: TcpListener, connection: PgConnection) {

    let connection = Arc::new(connection);

    let app = Router::new()
        .route("/health_check",get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection.clone());


    axum::serve(listener, app)
        .await
        .unwrap()
}   