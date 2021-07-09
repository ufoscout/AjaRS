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
    path: &'static str,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl <I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Rest<I, O> {
    
    pub fn builder(method: HttpMethod, path: &'static str) -> Self {
        Self {
            method,
            path,
            input: PhantomData,
            output: PhantomData,
        }
    }

    pub fn get(path: &'static str) -> Self {
        Rest::builder(HttpMethod::GET, path)
    }
    
    pub fn post(path: &'static str) -> Self {
        Rest::builder(HttpMethod::POST, path)
    }
}
