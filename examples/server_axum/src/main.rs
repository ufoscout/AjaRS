use ajars::axum::AxumHandler;
use axum::{
    body::{Body, BoxBody},
    http::{Method, Response, Uri},
    response::IntoResponse,
    Router,
};
use chrono::Local;
use examples_common::{hello::*, ping::*};
use std::{fmt::Display, net::SocketAddr};

/// The body type `PingRequest` and the result type `PingResult` are enforded at compile time
async fn ping(body: PingRequest, uri: Uri, method: Method) -> Result<PingResponse, ServerError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("ping - Request body: {:?}", body);
    Ok(PingResponse { message: format!("PONG - {}", Local::now()) })
}

/// The body type `HelloRequest` and the result type `HelloResponse` are enforded at compile time
async fn hello(body: HelloRequest, uri: Uri, method: Method) -> Result<HelloResponse, ServerError> {
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
