use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/desserts", get(desserts::list_desserts));

    let addr = "0.0.0.0:8080";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {addr}");

    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}
