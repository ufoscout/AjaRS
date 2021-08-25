
pub mod actix_web;
pub mod api;
pub mod axum;
pub mod error;

use ajars::{Rest, actix_web::actix_web::{App, HttpServer, web::{self, Json}}};
use ajars::actix_web::ActixWebHandler;
use ajars::reqwest::reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

// This defines a 'POST' call with path /ping, request type 'PingRequest' and response type 'PingResponse'
pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");

pub fn server() {
    HttpServer::new(move || 
            App::new().service(
                PING.to(ping)
            )
        );
    // start the server here...
}

async fn ping(_body: PingRequest) -> Result<PingResponse, Error> {
    Ok(PingResponse { message: "PONG".to_owned() })
}

pub async fn client() {
    let client = ClientBuilder::new().build().unwrap();

    let url = "http://127.0.0.1:8080/ping";           // Duplicated '/ping' path definition
    
    client.post(url)                  // Duplicated HTTP Post method definition
    
    .json(&PingRequest { message: "PING".to_owned() }) // Duplicated request type. Not checked at compile time
    
    .send().await.unwrap()
    .json::<PingResponse>().await.unwrap();      // Duplicated response type. Not checked at compile time
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub message: String,
}
