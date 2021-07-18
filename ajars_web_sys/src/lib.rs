use std::fmt::Display;
use std::marker::PhantomData;

use ajars_core::HttpMethod;
use ajars_core::RestType;

use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsCast as _;
use wasm_bindgen_futures::JsFuture;

use web_sys::window;
use web_sys::Headers;
use web_sys::Request as WebRequest;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response as WebResponse;
use web_sys::Window;

use error::Error;

use crate::error::WebError;

pub mod error;

#[derive(Debug, Clone, Copy)]
pub struct HttpStatus(u16);

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for HttpStatus {
    fn from(status: u16) -> Self {
        Self(status)
    }
}

impl HttpStatus {

    #[inline]
    pub fn status(&self) -> u16 {
        self.0
    }

    #[inline]
    pub fn is_informational(&self) -> bool {
        200 > self.0 && self.0 >= 100
    }

    /// Check if status is within 200-299.
    #[inline]
    pub fn is_success(&self) -> bool {
        300 > self.0 && self.0 >= 200
    }

    /// Check if status is within 300-399.
    #[inline]
    pub fn is_redirection(&self) -> bool {
        400 > self.0 && self.0 >= 300
    }

    /// Check if status is within 400-499.
    #[inline]
    pub fn is_client_error(&self) -> bool {
        500 > self.0 && self.0 >= 400
    }

    /// Check if status is within 500-599.
    #[inline]
    pub fn is_server_error(&self) -> bool {
        600 > self.0 && self.0 >= 500
    }
}

#[derive(Clone)]
pub struct AjarsWebSys {
    window: Window,
    base_url: String,
}

impl AjarsWebSys {
    pub fn new<P: Into<String>>(base_url: P) -> Result<Self, Error> {
        let window = window().ok_or_else(|| Error::MissingWindow)?;
        Ok(Self { window, base_url: base_url.into() })
    }

    pub fn request<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>(
        &'a self,
        rest: &'a REST,
    ) -> RequestBuilder<'a, I, O, REST> {
        let url = format!("{}{}", &self.base_url, rest.path());

        RequestBuilder { rest, window: &self.window, url, phantom_i: PhantomData, phantom_o: PhantomData }
    }
}

pub struct RequestBuilder<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>> {
    rest: &'a REST,
    window: &'a Window,
    url: String,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>
    RequestBuilder<'a, I, O, REST>
{
    
    /// Sends the Request to the target URL, returning a
    /// future Response.
    pub async fn send(self, data: &I) -> Result<O, Error> {

        let headers = Headers::new()
        .map_err(|err| Error::Builder{
            context: "Failed to create Headers object".to_owned(),
            source: WebError(format!("{:?}", err))
        })?;

        headers.append("Content-Type", "application/json").map_err(|err| Error::Builder{
            context: "Failed to append Context-Type header".to_owned(),
            source: WebError(format!("{:?}", err))
        })?;

        let mut opts = RequestInit::new();
        opts.mode(RequestMode::Cors);
        opts.headers(&headers);
        
        let mut uri = self.url;

        match self.rest.method() {
            HttpMethod::DELETE => {
                as_query_string(&mut opts, &mut uri, "DELETE", data)?;
            }
            HttpMethod::GET => {
                as_query_string(&mut opts, &mut uri, "GET", data)?;
            }
            HttpMethod::POST => {
                as_body(&mut opts, "POST", data)?;
            },
            HttpMethod::PUT => {
                as_body(&mut opts, "PUT", data)?;
            },
        };

        let request = WebRequest::new_with_str_and_init(&uri, &opts)
        .map_err(|err| Error::Builder{
            context: format!("Failed to create request for {}", uri.to_string()),
            source: WebError(format!("{:?}", err))
        })?;

        do_web_request(&self.window, request).await
        
    }

}

fn as_query_string<I: Serialize + DeserializeOwned>(opts: &mut RequestInit, uri: &mut String, method: &str, data: &I) -> Result<(), Error>{
    opts.method(method);
    uri.push_str("?");
    uri.push_str(&serde_urlencoded::to_string(data)
    .map_err(|err| Error::Builder{
        context: "Failed to serialize data as query string".to_owned(),
        source: WebError(format!("{:?}", err))
    })?);
    Ok(())
}

fn as_body<I: Serialize + DeserializeOwned>(opts: &mut RequestInit, method: &str, data: &I) -> Result<(), Error>{
    opts.method(method);
    opts.body(Some(&serde_wasm_bindgen::to_value(&data)
    .map_err(|err| Error::Builder {
        context: "Failed to serialize data as JSON body".to_owned(),
        source: WebError(format!("{:?}", err))
    })?));
    Ok(())
}

async fn do_web_request<O: Serialize + DeserializeOwned>(client: &Window, request: WebRequest) -> Result<O, Error> {

    let response = JsFuture::from(client.fetch_with_request(&request))
        .await
        .map_err(|err| Error::Builder{
            context: "Failed to issue request".to_owned(),
            source: err.into(),
        })?;

    let response = response
        .dyn_into::<WebResponse>()
        .map_err(|err| Error::Builder{
            context: "Future did not resolve into a web-sys Response".to_owned(),
            source: err.into(),
        })?;


    into_http_response(response).await
}

async fn into_http_response<O: Serialize + DeserializeOwned>(response: WebResponse) -> Result<O, Error> {

    let status = HttpStatus::from(response.status());

    // This 'if' check is how it is performed by Reqwest
    if status.is_client_error() || status.is_server_error() {
        Err(Error::Response {
            status,
            context: format!("Error HTTP status code received: {}", status.status()),
            source: WebError(format!("{:?}", response))
        },)
    } else {
        let value = JsFuture::from(response.json()
        .map_err(|err| Error::response(status, "Failed to read JSON body", err))?).await
        .map_err(|err| Error::response(status,"Failed to read JSON body", err))?;
    
        let data: O = serde_wasm_bindgen::from_value(value)
        .map_err(|err| Error::Builder {
            context: "Failed to deserialize data as JSON body from Response".to_owned(),
            source: WebError(format!("{:?}", err))
        })?;
    
        Ok(data)
    }

}