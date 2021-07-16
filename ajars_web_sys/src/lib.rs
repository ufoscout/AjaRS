use std::marker::PhantomData;

use ajars_core::HttpMethod;
use ajars_core::RestType;

use http::response::Builder;
use http::Response;
use http::StatusCode;

use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::JsCast as _;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

use web_sys::window;
use web_sys::Headers;
use web_sys::Request as WebRequest;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response as WebResponse;
use web_sys::Window;

use error::Error;

pub mod error;


/// Create a `http::Response` from one produced by the Fetch API.
async fn into_http_response<O: Serialize + DeserializeOwned>(response: WebResponse) -> Result<Response<O>, Error> {
    let status = response.status();
    let status = StatusCode::from_u16(status)?;

    /*
    {

        let buffer = response.array_buffer().map_err(|err| Error::web("failed to read HTTP body as ArrayBuffer", err))?;
        let buffer =
        JsFuture::from(buffer).await.map_err(|err| Error::web("failed to retrieve HTTP body from response", err))?;
        let buffer = buffer
        .dyn_into::<ArrayBuffer>()
        .map_err(|err| Error::web("future did not resolve into an js-sys ArrayBuffer", err))?;
        let length = buffer.byte_length() as usize;
        
        let data_view = DataView::new(&buffer, 0, length);
        let body = (0..length).fold(Vec::with_capacity(length), |mut body, i| {
            body.push(data_view.get_uint8(i));
            body
        });
        
        let data: O = serde_json::from_slice(&body).expect("Should build from JSON with serde");
    }
    */

    let value = JsFuture::from(response.json()
    .map_err(|err| Error::web("Failed to read JSON body", err))?).await
    .map_err(|err| Error::web("Failed to read JSON body", err))?;

    let data: O = serde_wasm_bindgen::from_value(value).expect("Should build from JSON with serde_wasm_bindgen");

    // TODO: We should also set headers and various other fields.
    let response = Builder::new().status(status).body(data)?;
    Ok(response)
}

async fn do_web_request<O: Serialize + DeserializeOwned>(client: &Window, request: WebRequest) -> Result<Response<O>, Error> {
    let response = JsFuture::from(client.fetch_with_request(&request))
        .await
        .map_err(|err| Error::web("failed to issue request", err))?;
    let response = response
        .dyn_into::<WebResponse>()
        .map_err(|err| Error::web("future did not resolve into a web-sys Response", err))?;

    into_http_response(response).await
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

        let headers = Headers::new().map_err(|err| Error::web("failed to create Headers object", err)).unwrap();
        headers.append("Content-Type", "application/json").expect("Should be able to add a header");

        let mut opts = RequestInit::new();
        opts.mode(RequestMode::Cors);
        opts.headers(&headers);
        
        let mut uri = self.url;

        match self.rest.method() {
            HttpMethod::DELETE => {
                opts.method("DELETE");
                uri.push_str("?");
                uri.push_str(&serde_urlencoded::to_string(data).unwrap());
            }
            HttpMethod::GET => {
                opts.method("GET");
                uri.push_str("?");
                uri.push_str(&serde_urlencoded::to_string(data).unwrap());
            }
            HttpMethod::POST => {
                opts.method("POST");
                opts.body(Some(&JsValue::from_str(&serde_json::to_string(data).unwrap())));
            },
            HttpMethod::PUT => {
                opts.method("PUT");
                opts.body(Some(&JsValue::from_str(&serde_json::to_string(data).unwrap())));
            },
        };

        let request = WebRequest::new_with_str_and_init(&uri, &opts)
        .map_err(|err| Error::web(format!("failed to create request for {}", uri.to_string()), err)).expect("WebRequest::new_with_str_and_init problem");

        let response = do_web_request(&self.window, request).await?;
        Ok(response.into_body())
        
    }

}
