mod server;
mod user;

use axum::handler;

use axum::{response::Html, routing::get, Router};

const INDEX: &str = include_str!("./index.html");

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(crate::server::handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
