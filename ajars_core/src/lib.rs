use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub enum HttpMethod {
    DELETE,
    GET,
    POST,
    PUT,
}

pub trait Rest<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> {

    fn new<P: Into<String>>(method: HttpMethod, path: P) -> RestImpl<I, O> {
        RestImpl::new(method, path)
    }

    fn path(&self) -> &str;
    fn method(&self) -> &HttpMethod;
}

pub struct RestConst<I, O> {
    path: &'static str,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl<I, O> Clone
    for RestConst<I, O>
{
    fn clone(&self) -> Self {
        Self {
            path: self.path,
            method: self.method.clone(),
            input: PhantomData,
            output: PhantomData,
        }
    }
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Rest<I,O>
    for RestConst<I, O>
{
    fn path(&self) -> &str {
        self.path
    }

    fn method(&self) -> &HttpMethod {
        &self.method
    }
}

impl<I, O> RestConst<I, O> {
    pub const fn new(method: HttpMethod, path: &'static str) -> Self {
        Self {
            method,
            path,
            input: PhantomData,
            output: PhantomData,
        }
    }

    pub fn delete(path: &'static str) -> Self {
        Self::new(HttpMethod::DELETE, path)
    }

    pub fn get(path: &'static str) -> Self {
        Self::new(HttpMethod::GET, path)
    }

    pub fn post(path: &'static str) -> Self {
        Self::new(HttpMethod::POST, path)
    }

    pub fn put(path: &'static str) -> Self {
        Self::new(HttpMethod::PUT, path)
    }
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
        Self::new(HttpMethod::DELETE, path)
    }

    pub fn get<P: Into<String>>(path: P) -> Self {
        Self::new(HttpMethod::GET, path)
    }

    pub fn post<P: Into<String>>(path: P) -> Self {
        Self::new(HttpMethod::POST, path)
    }

    pub fn put<P: Into<String>>(path: P) -> Self {
        Self::new(HttpMethod::PUT, path)
    }
}