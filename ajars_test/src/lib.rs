
pub mod actix_web;
pub mod api;
pub mod axum;
pub mod error;

use ajars::reqwest::reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

pub async fn do_somethig() {
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
