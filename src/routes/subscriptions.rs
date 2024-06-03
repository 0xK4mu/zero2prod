use serde::Deserialize;
use axum::Form;

#[derive(Deserialize, Debug)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(Form(form_data): Form<FormData>) {
    
}