use axum::{routing::get, Router};

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}
