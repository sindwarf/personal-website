use axum::{
    http::StatusCode, response::{Html, IntoResponse}, routing::{ get, get_service, Router }, Json
};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;
use tracing::{info, Level};
use tracing_subscriber;
use serde_derive::Serialize;

#[tokio::main]
async fn main() {
    // initialize tracing for logging
    // This code initializes the tracing subscriber with a maximum log level of INFO. This will allow us to log messages at the INFO level and above.
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    // TODO: Probably don't use relative path directory
    let app = Router::new()
        .route("/api/", get(Html("<h1>api Route</h1>")))
        .route("/api/test", get(api_test))
        .fallback(get_service(ServeDir::new("../client/dist/spa")).handle_error(|_| async move {
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        }));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

#[derive(Serialize)]
struct Test {
    name: String,
    description: String
}

async fn api_test() -> Json<Test>{
    let test_response = Test {
        name: String::from("FirstLastName"),
        description: String::from("Description"),
    };

    Json(test_response)
}
