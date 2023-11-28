use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde::Serialize;

#[derive(Clone)]
pub enum HttpMethod {
    DELETE,
    GET,
    POST,
    PUT,
}

pub trait RestType<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> {
    fn build<P: Into<String>>(method: HttpMethod, path: P) -> RestFluent<I, O> {
        RestFluent::new(method, path)
    }

    fn path(&self) -> &str;
    fn method(&self) -> &HttpMethod;
}

pub struct Rest<I, O> {
    path: &'static str,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl<I, O> Clone for Rest<I, O> {
    fn clone(&self) -> Self {
        Self { path: self.path, method: self.method.clone(), input: PhantomData, output: PhantomData }
    }
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> RestType<I, O> for Rest<I, O> {
    fn path(&self) -> &str {
        self.path
    }

    fn method(&self) -> &HttpMethod {
        &self.method
    }
}

impl<I, O> Rest<I, O> {
    pub const fn new(method: HttpMethod, path: &'static str) -> Self {
        Self { method, path, input: PhantomData, output: PhantomData }
    }

    pub const fn delete(path: &'static str) -> Self {
        Self::new(HttpMethod::DELETE, path)
    }

    pub const fn get(path: &'static str) -> Self {
        Self::new(HttpMethod::GET, path)
    }

    pub const fn post(path: &'static str) -> Self {
        Self::new(HttpMethod::POST, path)
    }

    pub const fn put(path: &'static str) -> Self {
        Self::new(HttpMethod::PUT, path)
    }
}

pub struct RestFluent<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> {
    path: String,
    method: HttpMethod,
    input: PhantomData<I>,
    output: PhantomData<O>,
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Clone for RestFluent<I, O> {
    fn clone(&self) -> Self {
        Self { path: self.path.clone(), method: self.method.clone(), input: PhantomData, output: PhantomData }
    }
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> RestType<I, O> for RestFluent<I, O> {
    fn path(&self) -> &str {
        &self.path
    }

    fn method(&self) -> &HttpMethod {
        &self.method
    }
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> RestFluent<I, O> {
    pub fn new<P: Into<String>>(method: HttpMethod, path: P) -> Self {
        Self { method, path: path.into(), input: PhantomData, output: PhantomData }
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
