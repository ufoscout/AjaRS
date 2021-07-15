use std::marker::PhantomData;

use ajars_core::HttpMethod;
use ajars_core::RestType;
use bytes::Bytes;

use http::response::Builder;
use http::Request;
use http::Response;
use http::StatusCode;

use js_sys::ArrayBuffer;
use js_sys::DataView;

use serde::Serialize;
use serde::de::DeserializeOwned;
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

/// Convert an `http::Request` into one as used by the Fetch API.
fn into_web_request(request: Request<Option<String>>) -> Result<WebRequest, Error> {
  let (parts, body) = request.into_parts();
  let headers = Headers::new().map_err(|err| Error::web("failed to create Headers object", err))?;
  let headers =
    parts
      .headers
      .iter()
      .try_fold::<_, _, Result<_, Error>>(headers, |headers, (k, v)| {
        let _ = headers.append(k.as_str(), v.to_str()?);
        Ok(headers)
      })?;

  let value;
  let body = if let Some(body) = body {
    value = JsValue::from_str(&body);
    Some(&value)
  } else {
    None
  };
  let uri = parts.uri;

  let mut opts = RequestInit::new();
  opts.mode(RequestMode::Cors);
  opts.method(parts.method.as_str());
  opts.headers(&headers);
  opts.body(body);

  let request = WebRequest::new_with_str_and_init(&uri.to_string(), &opts).map_err(|err| {
    Error::web(
      format!("failed to create request for {}", uri.to_string()),
      err,
    )
  })?;

  Ok(request)
}


/// Create a `http::Response` from one produced by the Fetch API.
async fn into_http_response(response: WebResponse) -> Result<Response<Bytes>, Error> {
  let status = response.status();
  let status = StatusCode::from_u16(status)?;

  let buffer = response
    .array_buffer()
    .map_err(|err| Error::web("failed to read HTTP body as ArrayBuffer", err))?;
  let buffer = JsFuture::from(buffer)
    .await
    .map_err(|err| Error::web("failed to retrieve HTTP body from response", err))?;
  let buffer = buffer
    .dyn_into::<ArrayBuffer>()
    .map_err(|err| Error::web("future did not resolve into an js-sys ArrayBuffer", err))?;
  let length = buffer.byte_length() as usize;

  let data_view = DataView::new(&buffer, 0, length);
  let body = (0..length).fold(Vec::with_capacity(length), |mut body, i| {
    body.push(data_view.get_uint8(i));
    body
  });
  let bytes = Bytes::from(body);

  // TODO: We should also set headers and various other fields.
  let response = Builder::new().status(status).body(bytes)?;
  Ok(response)
}

async fn do_request(
  client: &Window,
  request: Request<Option<String>>,
) -> Result<Response<Bytes>, Error> {
  let request = into_web_request(request)?;
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
    pub fn new(base_url: String) -> Result<Self, Error> {
        let window = window().ok_or_else(|| Error::MissingWindow)?;
        Ok(Self { window, base_url })
    }
    
    pub fn request<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>(
        &self,
        rest: &'a REST,
    ) -> RequestBuilder<'a, I, O, REST> {
        let url = format!("{}{}", &self.base_url, rest.path());

        let request_builder = Request::builder();
        RequestBuilder { rest, url, request_builder, phantom_i: PhantomData, phantom_o: PhantomData }
    }
}

pub struct RequestBuilder<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>> {
    rest: &'a REST,
    url: String,
    request_builder: http::request::Builder,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl <'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>> 
    RequestBuilder<'a, I, O, REST> {


}

/// An HTTP client for usage in WASM environments.
#[derive(Debug)]
pub struct Client(Window);

impl Client {
  /// Create a new WASM HTTP client.
  pub fn new() -> Self {
    let window = window().expect("no window found; not running inside a browser?");
    Self(window)
  }

  /// Issue a request and retrieve a response.
  pub async fn request(&self, request: Request<Option<String>>) -> Result<Response<Bytes>, Error> {
    do_request(&self.0, request).await
  }
}

impl Default for Client {
  fn default() -> Self {
    Self::new()
  }
}

impl From<Window> for Client {
  /// Create a `Client` from a `Window`.
  fn from(window: Window) -> Self {
    Self(window)
  }
}

impl Into<Window> for Client {
  /// Extract the `Window` from a `Client`.
  fn into(self) -> Window {
    self.0
  }
}
