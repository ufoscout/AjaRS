use ajars::Rest;
use serde::{Serialize, Deserialize};

/// A "Ping" REST call
pub const PING: Rest::<PingRequest, PingResponse> = Rest::get("/api/ping");

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub message: String
}
