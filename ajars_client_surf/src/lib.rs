use std::marker::PhantomData;

use ajars_core::{HttpMethod, RestType};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::surf::{Client, RequestBuilder as SurfRequestBuilder};

pub mod surf {
    pub use ::surf::*;
}

#[derive(Clone)]
pub struct AjarsSurf {
    client: Client,
    base_url: String,
}

impl AjarsSurf {
    pub fn new<S: Into<String>>(client: Client, base_url: S) -> Self {
        Self { client, base_url: base_url.into() }
    }

    pub fn request<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>(
        &self,
        rest: &'a REST,
    ) -> RequestBuilder<'a, I, O, REST> {
        let url = format!("{}{}", self.base_url, rest.path());

        let request = match rest.method() {
            HttpMethod::DELETE => self.client.delete(&url),
            HttpMethod::GET => self.client.get(&url),
            HttpMethod::POST => self.client.post(&url).header("Content-Type", "application/json"),
            HttpMethod::PUT => self.client.put(&url).header("Content-Type", "application/json"),
        };

        RequestBuilder { rest, request, phantom_i: PhantomData, phantom_o: PhantomData }
    }
}

pub struct RequestBuilder<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>> {
    rest: &'a REST,
    request: SurfRequestBuilder,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>
    RequestBuilder<'a, I, O, REST>
{
    /// Sends the Request to the target URL, returning a
    /// future Response.
    pub async fn send(self, data: &I) -> Result<O, surf::Error> {
        let request = match self.rest.method() {
            HttpMethod::DELETE | HttpMethod::GET => self.request.query(data)?,
            HttpMethod::POST | HttpMethod::PUT => {
                self.request.header("Content-Type", "application/json").body(surf::Body::from_json(data)?)
            }
        };

        request.send().await?.body_json().await
    }
}
