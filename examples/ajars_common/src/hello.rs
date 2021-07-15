use ajars::Rest;
use serde::{Serialize, Deserialize};

/// A "Hello" REST call
pub const HELLO: Rest::<HelloRequest, HelloResponse> = Rest::post("/api/hello");

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloRequest {
    pub names: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloResponse {
    pub hellos: Vec<String>
}