use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};
use crate::surf::Client;

pub mod surf {
    pub use surf::*;
}

#[derive(Clone)]
pub struct RestSurf {
    client: Client,
    base_url: String,
}

impl RestSurf {
    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    pub async fn submit<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>(
        &self,
        rest: &REST,
        data: &I,
    ) -> Result<O, surf::Error> {
        let url = format!("{}{}", self.base_url, rest.path());

        let request = match rest.method() {
            HttpMethod::DELETE => self.client.delete(&url).query(data)?,
            HttpMethod::GET => self.client.get(&url).query(data)?,
            HttpMethod::POST => self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .body(surf::Body::from_json(data)?),
            HttpMethod::PUT => self.client
                .put(&url)
                .header("Content-Type", "application/json")
                .body(surf::Body::from_json(data)?),
        };

        request.send().await?.body_json().await
    }
}
