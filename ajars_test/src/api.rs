use std::collections::HashMap;

use ajars::Rest;
use serde::{Deserialize, Serialize};

// This defines a 'DELETE' call with request type 'InfoRequest' and response type 'InfoResponse'
pub const INFO_DELETE: Rest<InfoRequest<String>, InfoResponse<String>> = Rest::delete("/api/info");

// This defines a 'GET' call with request type 'InfoRequest' and response type 'InfoResponse'
pub const INFO_GET: Rest<InfoRequest<String>, InfoResponse<String>> = Rest::get("/api/info");

// This defines a 'POST' call with request type 'InfoRequest' and response type 'InfoResponse'
pub const INFO_POST: Rest<InfoRequest<String>, InfoResponse<String>> = Rest::post("/api/info");

// This defines a 'PUT' call with request type 'InfoRequest' and response type 'InfoResponse'
pub const INFO_PUT: Rest<InfoRequest<String>, InfoResponse<String>> = Rest::put("/api/info");

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoRequest<T> {
    pub payload: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InfoResponse<T> {
    pub request_headers: HashMap<String, String>,
    pub request_method: String,
    pub request_query_string: String,
    pub request_path: String,
    pub request_payload: T,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Simple<O> {
    pub inner: O,
}
