use actix_web::{App, HttpRequest, HttpServer, ResponseError};
use ajars::actix_web::ActixWebHandler;
use chrono::Local;
use examples_common::{hello::*, ping::*};
use std::fmt::Display;

/// The body type `PingRequest` and the result type `PingResult` are enforded at compile time
async fn ping(body: PingRequest, request: HttpRequest) -> Result<PingResponse, ServerError> {
    println!("ping - Request path: {:?}", request.path());
    println!("ping - Request method: {:?}", request.method());
    println!("ping - Request query_string: {:?}", request.query_string());
    println!("ping - Request body: {:?}", body);
    Ok(PingResponse { message: format!("PONG - {}", Local::now()) })
}

/// The body type `HelloRequest` and the result type `HelloResponse` are enforded at compile time
async fn hello(body: HelloRequest, request: HttpRequest) -> Result<HelloResponse, ServerError> {
    println!("hello - Request path: {:?}", request.path());
    println!("hello - Request method: {:?}", request.method());
    println!("hello - Request query_string: {:?}", request.query_string());
    println!("hello - Request body: {:?}", body);
    Ok(HelloResponse { hellos: body.names.iter().map(|name| format!("Hello, {}!", name)).collect() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Starting actix server at port {}", port);

    HttpServer::new(|| {
        App::new()
            .service(PING.to(ping)) // This creates a GET /api/ping endpoint
            .service(HELLO.to(hello)) // This creates a POST /api/hello endpoint
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

#[derive(Debug, Clone)]
struct ServerError {}

impl Display for ServerError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl ResponseError for ServerError {}
