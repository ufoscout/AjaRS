use actix_web::{
    web::{Data, Json},
    App, HttpRequest, HttpServer, ResponseError,
};
use ajars::actix_web::HandleActix;
use ajars_common::{hello::*, ping::*};
use chrono::Local;
use std::fmt::Display;

/// The body type `PingRequest` and the result type `PingResult` are enforded at compile time
async fn ping(request: HttpRequest, _data: Data<()>, body: PingRequest) -> Result<Json<PingResponse>, ServerError> {
    println!("ping - Request path: {:?}", request.path());
    println!("ping - Request method: {:?}", request.method());
    println!("ping - Request query_string: {:?}", request.query_string());
    println!("ping - Request body: {:?}", body);
    Ok(Json(PingResponse { message: format!("PONG - {}", Local::now()) }))
}

/// The body type `HelloRequest` and the result type `HelloResponse` are enforded at compile time
async fn hello(request: HttpRequest, _data: Data<()>, body: HelloRequest) -> Result<Json<HelloResponse>, ServerError> {
    println!("hello - Request path: {:?}", request.path());
    println!("hello - Request method: {:?}", request.method());
    println!("hello - Request query_string: {:?}", request.query_string());
    println!("hello - Request body: {:?}", body);
    Ok(Json(HelloResponse { hellos: body.names.iter().map(|name| format!("Hello, {}!", name)).collect() }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(Data::new(()))
            .service(PING.handle(ping)) // This creates a GET /api/ping endpoint
            .service(HELLO.handle(hello)) // This creates a POST /api/hello endpoint
    })
    .bind(("127.0.0.1", 8080))?
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