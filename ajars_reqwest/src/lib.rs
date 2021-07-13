use ajars_core::{HttpMethod, Rest};
use reqwest::Client;
use serde::{de::DeserializeOwned, Serialize};

#[derive(Clone)]
pub struct RestReqwest {
    client: Client,
    base_url: String,
}

impl RestReqwest {

    pub fn new(client: Client, base_url: String) -> Self {
        Self { client, base_url }
    }

    pub async fn submit<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: Rest<I, O>>(
        &self,
        rest: &REST,
        data: &I,
    ) -> Result<O, reqwest::Error> {
        let url = format!("{}{}", &self.base_url, rest.path());

        let request = match rest.method() {
            HttpMethod::DELETE => self.client.delete(&url).query(data),
            HttpMethod::GET => self.client.get(&url).query(data),
            HttpMethod::POST => self.client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(data),
            HttpMethod::PUT => self.client
                .put(&url)
                .header("Content-Type", "application/json")
                .json(data),
        };

        request.send().await?.json().await
    }
}
