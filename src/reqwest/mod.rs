use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};

use crate::Rest;

impl <I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned> Rest<I, O> {

    pub async fn reqwest(&self, client: &Client, data: &I) -> Result<O, reqwest::Error> {

        let req = match self.method {
            crate::HttpMethod::GET => client.get(self.path),
            crate::HttpMethod::POST => client.post(self.path),
        };

        req
        .json(data)
        .send()
        .await?
        .json()
        .await
    }

}