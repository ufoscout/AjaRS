use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

#[cfg(feature = "server_actix_web")]
pub mod actix_web;
#[cfg(feature = "client_reqwest")]
pub mod reqwest;
#[cfg(feature = "client_surf")]
pub mod surf;

#[derive(Clone)]
pub enum HttpMethod {
    DELETE,
    GET,
    POST,
    PUT,
}


pub trait Rest<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> {
    fn path(&self) -> &str;
    fn method(&self) -> &HttpMethod;
}

pub struct RestImpl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> {
    path: String,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Clone
    for RestImpl<I, O>
{
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            method: self.method.clone(),
            input: PhantomData,
            output: PhantomData,
        }
    }
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Rest<I,O>
    for RestImpl<I, O>
{
    fn path(&self) -> &str {
        &self.path
    }

    fn method(&self) -> &HttpMethod {
        &self.method
    }
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> RestImpl<I, O> {
    pub fn new<P: Into<String>>(method: HttpMethod, path: P) -> Self {
        Self {
            method,
            path: path.into(),
            input: PhantomData,
            output: PhantomData,
        }
    }

    pub fn delete<P: Into<String>>(path: P) -> Self {
        RestImpl::new(HttpMethod::DELETE, path)
    }

    pub fn get<P: Into<String>>(path: P) -> Self {
        RestImpl::new(HttpMethod::GET, path)
    }

    pub fn post<P: Into<String>>(path: P) -> Self {
        RestImpl::new(HttpMethod::POST, path)
    }

    pub fn put<P: Into<String>>(path: P) -> Self {
        RestImpl::new(HttpMethod::PUT, path)
    }
}
