use std::{collections::HashMap, net::SocketAddr};

use ajars::{RestType, axum::{AxumRoute, axum::{self, http::{self, Response}, Router, body::Body, response::IntoResponse}}};

use crate::{api::*, error::MyError};

impl IntoResponse for MyError {
        
    type Body = Body;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
        Response::new(Body::empty())
    }

}

async fn echo(body: Simple<String>, uri: http::Uri, method: http::Method) -> Result<Simple<String>, MyError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("echo - Request body: {:?}", body);
    Ok(body)
}

async fn info(body: InfoRequest<String>, uri: http::Uri, method: http::Method, headers: http::HeaderMap) -> Result<InfoResponse<String>, MyError> {
    println!("echo - Request path: {:?}", uri.path());
    println!("echo - Request method: {:?}", method);
    println!("echo - Request query_string: {:?}", uri.query());
    println!("echo - Request body: {:?}", body);

    let mut request_headers = HashMap::new();

    for (name, value) in headers {
        request_headers.insert(name.map(|header| header.to_string()).unwrap_or_default(), value.to_str().expect("Header value should be a string").to_owned());
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
            .or(echo_rest.route(echo))
            .or(INFO_DELETE.route(info))
            .or(INFO_GET.route(info))
            .or(INFO_POST.route(info))
            .or(INFO_PUT.route(info));
        
        let addr = SocketAddr::from(([127, 0, 0, 1], free_port));
        
        println!("Start axum to {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();

    });
    free_port
}