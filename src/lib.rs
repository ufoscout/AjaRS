use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "server_actix_web")]
pub mod actix_web;
#[cfg(feature = "client_reqwest")]
pub mod reqwest;

#[derive(Clone)]
pub enum HttpMethod {
    GET,
    POST
}

#[derive(Clone)]
pub struct Rest<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> {
    path: String,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl <I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Rest<I, O> {
    
    pub fn builder<P: Into<String>>(method: HttpMethod, path: P) -> Self {
        Self {
            method,
            path: path.into(),
            input: PhantomData,
            output: PhantomData,
        }
    }

    pub fn get<P: Into<String>>(path: P) -> Self {
        Rest::builder(HttpMethod::GET, path)
    }
    
    pub fn post<P: Into<String>>(path: P) -> Self {
        Rest::builder(HttpMethod::POST, path)
    }
}
