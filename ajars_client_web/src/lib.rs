use std::collections::HashMap;
use std::fmt::Display;
use std::marker::PhantomData;
use std::rc::Rc;

use ajars_core::{HttpMethod, RestType};
use error::Error;
use gloo_net::http::{Request, Response};
use gloo_utils::window;
use http::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use web_sys::RequestMode;

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

/// Allows to modify and inspect a Request/Response
pub trait Interceptor {
    /// Called before a request is performed
    fn before_request(&self, _uri: &str, request: Request) -> Result<Request, Error> {
        Ok(request)
    }

    /// Called after a response is received and before the body is consumed
    fn after_response(&self, response: Result<Response, Error>) -> Result<Response, Error> {
        response
    }
}

/// An Interceptor implementation that does not alter the request/response
pub struct DoNothingInterceptor {}

impl Interceptor for DoNothingInterceptor {}

pub struct AjarsClientWeb {
    interceptor: Rc<dyn Interceptor>,
    base_url: String,
}

impl AjarsClientWeb {
    pub fn new<P: Into<String>>(base_url: P) -> Result<AjarsClientWeb, Error> {
        AjarsClientWeb::new_with_interceptor(base_url, Rc::new(DoNothingInterceptor {}))
    }

    pub fn new_with_interceptor<P: Into<String>>(
        base_url: P,
        interceptor: Rc<dyn Interceptor>,
    ) -> Result<AjarsClientWeb, Error> {
        Ok(AjarsClientWeb { interceptor, base_url: base_url.into() })
    }

    pub fn request<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>(
        &'a self,
        rest: &'a REST,
    ) -> RequestBuilder<'a, I, O, REST> {
        let url = format!("{}{}", &self.base_url, rest.path());

        RequestBuilder::new(rest, url, self.interceptor.as_ref())
            .add_header("Content-Type", "application/json")
    }
}

pub struct RequestBuilder<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>> {
    rest: &'a REST,
    interceptor: &'a dyn Interceptor,
    headers: HashMap<String, String>,
    url: String,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<'a, I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, REST: RestType<I, O>>
    RequestBuilder<'a, I, O, REST>
{
    pub fn new(rest: &'a REST, url: String, interceptor: &'a dyn Interceptor) -> Self {
        RequestBuilder {
            rest,
            interceptor,
            url,
            headers: HashMap::new(),
            phantom_i: PhantomData,
            phantom_o: PhantomData,
        }
    }

    /// Add a header to the request
    pub fn add_header<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Enable HTTP basic authentication.
    pub fn basic_auth(self, username: &str, password: Option<&str>) -> Result<Self, Error> {
        let user_pass = format!("{}:{}", username, password.unwrap_or_default());
        let encoded_user_pass = window().btoa(&user_pass).map_err(|err| Error::Builder {
            context: "Failed to encode in base64 the basic auth string".to_owned(),
            error: format!("{:?}", err),
        })?;

        Ok(self.add_header("AUTHORIZATION", format!("Basic {}", encoded_user_pass)))
    }

    /// Enable HTTP bearer authentication.
    pub fn bearer_auth<T>(self, token: T) -> Self
    where
        T: std::fmt::Display,
    {
        let header_value = format!("Bearer {}", token);
        self.add_header("AUTHORIZATION", header_value)
    }

    /// Sends the Request to the target URL, returning a
    /// future Response.
    pub async fn send(self, data: &I) -> Result<O, Error> {

        let request = match self.rest.method() {
            HttpMethod::DELETE => as_query_string(&self.url, http::Method::DELETE, &self.headers, data),
            HttpMethod::GET => as_query_string(&self.url, http::Method::GET, &self.headers, data),
            HttpMethod::POST => as_body(&self.url, http::Method::POST, &self.headers, data),
            HttpMethod::PUT => as_body(&self.url, http::Method::PUT, &self.headers, data),
        }?;

        let request = self.interceptor.before_request(&self.url, request)?;

        let response = request.send().await.map_err(|err| Error::Builder {
            context: format!("Failed to send request"),
            error: format!("{:?}", err),
        });

        let response = self.interceptor.after_response(response)?;
        into_http_response(response).await
    }
}

fn as_query_string<I: Serialize + DeserializeOwned>(
    uri: &str,
    method: Method,
    headers: &HashMap<String, String>,
    data: &I,
) -> Result<gloo_net::http::Request, Error> {
    let mut uri = uri.to_owned();
    uri.push('?');
    uri.push_str(&serde_urlencoded::to_string(data).map_err(|err| Error::Builder {
        context: "Failed to serialize data as query string".to_owned(),
        error: format!("{:?}", err),
    })?);
    let mut request = gloo_net::http::RequestBuilder::new(&uri)
        .method(method)
        .mode(RequestMode::Cors);

    for (header_key, header_value) in headers {
        request = request.header(header_key, header_value);
    }

    request.build().map_err(|err| Error::Builder {
        context: "Failed to build Request".to_owned(),
        error: format!("{:?}", err),
    })
}

fn as_body<I: Serialize + DeserializeOwned>(
    uri: &str,
    method: Method,
    headers: &HashMap<String, String>,
    data: &I,
) -> Result<Request, Error> {
    let mut request = gloo_net::http::RequestBuilder::new(&uri)
        .method(method)
        .mode(RequestMode::Cors);
    
    for (header_key, header_value) in headers {
        request = request.header(header_key, header_value);
    }

    request.json(data).map_err(|err| Error::Builder {
        context: "Failed to serialize data as JSON body".to_owned(),
        error: format!("{:?}", err),
    })
}

async fn into_http_response<O: Serialize + DeserializeOwned>(response: Response) -> Result<O, Error> {
    let status = HttpStatus::from(response.status());

    // This 'if' check is how it is performed by Reqwest
    if status.is_client_error() || status.is_server_error() {
        Err(Error::Response {
            status,
            context: format!("Error HTTP status code received: {}", status.status()),
            error: format!("Status code error: {:?}", response),
        })
    } else {
        response.json().await.map_err(|err| Error::Response {
            status,
            context: format!("Failed to read JSON body: {}", status.status()),
            error: format!("{:?}", err),
        })
    }
}
