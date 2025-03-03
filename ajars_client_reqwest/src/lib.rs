use std::convert::TryFrom;
use std::marker::PhantomData;

use ::reqwest::header::{HeaderName, HeaderValue};
use ajars_core::{HttpMethod, RestType};
use http::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::reqwest::{Client, RequestBuilder as ReqwestRequestBuilder};

pub mod reqwest {
    pub use ::reqwest::*;
}

#[derive(Clone)]
pub struct AjarsClientReqwest {
    client: Client,
    base_url: String,
}

impl AjarsClientReqwest {
    pub fn new<S: Into<String>>(client: Client, base_url: S) -> Self {
        Self { client, base_url: base_url.into() }
    }

    pub fn request<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>(
        &self,
        rest: &'a REST,
    ) -> RequestBuilder<'a, I, O, REST> {
        let url = format!("{}{}", &self.base_url, rest.path());

        let request = match rest.method() {
            HttpMethod::DELETE => self.client.delete(&url),
            HttpMethod::GET => self.client.get(&url),
            HttpMethod::POST => self.client.post(&url),
            HttpMethod::PUT => self.client.put(&url),
        };

        RequestBuilder { rest, request, phantom_i: PhantomData, phantom_o: PhantomData }
    }
}

pub struct RequestBuilder<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>> {
    rest: &'a REST,
    request: ReqwestRequestBuilder,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>
    RequestBuilder<'_, I, O, REST>
{
    /// Sends the Request to the target URL, returning a
    /// future Response.
    pub async fn send(self, data: &I) -> Result<O, reqwest::Error> {
        let request = match self.rest.method() {
            HttpMethod::DELETE | HttpMethod::GET => self.request.query(data),
            HttpMethod::POST | HttpMethod::PUT => self.request.header("Content-Type", "application/json").json(data),
        };

        request.send().await?.json().await
    }

    /// Add a `Header` to this Request.
    pub fn header<K, V>(mut self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.request = self.request.header(key, value);
        self
    }

    /// Add a set of Headers to the existing ones on this Request.
    ///
    /// The headers will be merged in to any already set.
    pub fn headers(mut self, headers: HeaderMap) -> Self {
        self.request = self.request.headers(headers);
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Enable HTTP basic authentication.
    pub fn basic_auth<U, P>(mut self, username: U, password: Option<P>) -> Self
    where
        U: std::fmt::Display,
        P: std::fmt::Display,
    {
        self.request = self.request.basic_auth(username, password);
        self
    }

    /// Enable HTTP bearer authentication.
    pub fn bearer_auth<T>(mut self, token: T) -> Self
    where
        T: std::fmt::Display,
    {
        self.request = self.request.bearer_auth(token);
        self
    }

    #[cfg(not(target_arch = "wasm32"))]
    /// Enables a request timeout.
    ///
    /// The timeout is applied from when the request starts connecting until the
    /// response body has finished. It affects only this request and overrides
    /// the timeout configured using `ClientBuilder::timeout()`.
    pub fn timeout(mut self, timeout: std::time::Duration) -> Self {
        self.request = self.request.timeout(timeout);
        self
    }
}
