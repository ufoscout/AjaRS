use std::marker::PhantomData;

use serde::{Serialize, de::DeserializeOwned};

#[cfg(feature = "server_actix_web")]
mod actix_web;
#[cfg(feature = "client_reqwest")]
mod reqwest;

pub enum HttpMethod {
    GET,
    POST
}

pub struct Rest<I: DeserializeOwned, O: Serialize> {
    path: &'static str,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl <I: DeserializeOwned, O: Serialize> Rest<I, O> {
    
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

#[cfg(test)]
mod test {

    use super::*;

    fn rest() -> Rest<String, String> {
         Rest::get("/api/hello/world")
    }

    #[test]
    fn const_builder() {
        let rest = rest();
    }

}