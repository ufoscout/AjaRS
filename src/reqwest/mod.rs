use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

use crate::Rest;

impl<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Rest<I, O> {
    pub async fn reqwest(
        &self,
        client: &Client,
        base_url: &str,
        data: &I,
    ) -> Result<O, reqwest::Error> {
        let url = format!("{}{}", base_url, self.path);

        let request = match self.method {
            crate::HttpMethod::DELETE => client.delete(&url).query(data),
            crate::HttpMethod::GET => client.get(&url).query(data),
            crate::HttpMethod::POST => client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(data),
            crate::HttpMethod::PUT => client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(data),
        };

        request.send().await?.json().await
    }
}

#[derive(Clone)]
pub struct RestReqwest {
    client: Client,
    base_url: String,
}

impl RestReqwest {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    pub async fn submit<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned>(
        &self,
        rest: &Rest<I, O>,
        data: &I,
    ) -> Result<O, reqwest::Error> {
        rest.reqwest(&self.client, &self.base_url, data).await
    }
}
