use ajars::Rest;
use serde::{Deserialize, Serialize};

// This defines a 'POST' call with request type 'HelloRequest' and response type 'HelloResponse'
pub const HELLO: Rest<HelloRequest, HelloResponse> = Rest::post("/api/hello");

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloRequest {
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloResponse {
    pub hellos: Vec<String>,
}
