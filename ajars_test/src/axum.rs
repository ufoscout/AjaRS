use std::collections::HashMap;
use std::net::SocketAddr;

use ::axum::body::BoxBody;
use ::axum::extract::State;
use ajars::axum::axum::body::Body;
use ajars::axum::axum::http::{self, Response};
use ajars::axum::axum::response::IntoResponse;
use ajars::axum::axum::{self, Router};
use ajars::axum::AxumHandler;
use ajars::RestType;

use crate::api::*;
use crate::error::MyError;

impl IntoResponse for MyError {
    fn into_response(self) -> Response<BoxBody> {
        Response::new(axum::body::boxed(Body::empty()))
    }
}

async fn echo(uri: http::Uri, method: http::Method, body: Simple<String>) -> Result<Simple<String>, MyError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("echo - Request body: {:?}", body);
    Ok(body)
}

async fn info(
    _: State<()>,
    uri: http::Uri,
    method: http::Method,
    headers: http::HeaderMap,
    body: InfoRequest<String>,
) -> Result<InfoResponse<String>, MyError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("echo - Request body: {:?}", body);

    let mut request_headers = HashMap::new();

    for (name, value) in headers {
        request_headers.insert(
            name.map(|header| header.to_string()).unwrap_or_default(),
            value.to_str().expect("Header value should be a string").to_owned(),
        );
    }

    Ok(InfoResponse {
        request_headers,
        request_method: method.as_str().to_string(),
        request_path: uri.path().to_string(),
        request_payload: body.payload,
        request_query_string: uri.query().unwrap_or_default().to_string(),
    })
}

/// spanws an actix server and returns the server port
pub fn spawn_axum<REST: 'static + Clone + Send + RestType<Simple<String>, Simple<String>>>(echo_rest: REST) -> u16 {
    let free_port = port_check::free_local_port().unwrap();
    //let address = format!("127.0.0.1:{}", free_port);

    // Start Server
    tokio::spawn(async move {
        let app = Router::new()
            .merge(echo_rest.to(echo))
            .merge(INFO_DELETE.to(info))
            .merge(INFO_GET.to(info))
            .merge(INFO_POST.to(info))
            .merge(INFO_PUT.to(info));

        let addr = SocketAddr::from(([127, 0, 0, 1], free_port));

        println!("Start axum to {}", addr);

        axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
    });
    free_port
}
