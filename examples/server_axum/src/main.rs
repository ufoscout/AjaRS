use std::fmt::Display;
use std::net::SocketAddr;

use ajars::axum::AxumHandler;
use axum::body::{Body, BoxBody};
use axum::http::{Method, Response, Uri};
use axum::response::IntoResponse;
use axum::Router;
use chrono::Local;
use examples_common::hello::*;
use examples_common::ping::*;

/// The body type `PingRequest` and the result type `PingResult` are enforded at compile time
async fn ping(uri: Uri, method: Method, body: PingRequest) -> Result<PingResponse, ServerError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("ping - Request body: {:?}", body);
    Ok(PingResponse { message: format!("PONG - {}", Local::now()) })
}

/// The body type `HelloRequest` and the result type `HelloResponse` are enforded at compile time
async fn hello(uri: Uri, method: Method, body: HelloRequest) -> Result<HelloResponse, ServerError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("hello - Request body: {:?}", body);
    Ok(HelloResponse { hellos: body.names.iter().map(|name| format!("Hello, {}!", name)).collect() })
}

#[tokio::main]
async fn main() {
    let port = 8080;
    println!("Starting actix server at port {}", port);

    let app = Router::new().merge(PING.to(ping)).merge(HELLO.to(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("Start axum to {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.expect("Axum server should start");
}

#[derive(Debug, Clone)]
struct ServerError {}

impl Display for ServerError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response<BoxBody> {
        Response::new(axum::body::boxed(Body::empty()))
    }
}
