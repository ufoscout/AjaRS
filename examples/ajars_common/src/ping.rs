use ajars::Rest;
use serde::{Deserialize, Serialize};

// This defines a 'GET' call with request type 'PingRequest' and response type 'PingResponse'
pub const PING: Rest<PingRequest, PingResponse> = Rest::get("/api/ping");

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub message: String,
}
