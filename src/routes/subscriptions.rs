use serde::Deserialize;
use axum::Form;
use sqlx::PgConnection;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(
    Form(_form_data): Form<FormData>,
    _connection: Arc<PgConnection>
) {
    
}