use tokio::net::TcpListener;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    //On récupère le port assigné par l'OS
    let port = listener.local_addr().unwrap().port();
    println!("listening on port {}", port); 
    run(listener).await
}