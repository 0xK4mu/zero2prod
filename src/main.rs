use tokio::net::TcpListener;
use zero2prod::startup::run;
use zero2prod::configuration::get_configuration;
use sqlx::{Connection, PgConnection};

#[tokio::main]
async fn main() {
    // Panic si on ne peut pas lire la config
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(
        &configuration.database.connection_string()
    )
    .await
    .expect("Failed to connect to Postgres.");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind random port");
    //On récupère le port assigné par l'OS
    let port = listener.local_addr().unwrap().port();
    println!("listening on port {}", port); 
    run(listener, connection).await
}