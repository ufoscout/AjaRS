use std::collections::HashMap;

use actix_rt::spawn;
use ajars::{RestType, actix_web::{HandleActix, actix_web::{App, HttpRequest, HttpServer, ResponseError, web::{Data, Json}}}};

use crate::{api::*, error::MyError};

impl ResponseError for MyError {}


async fn echo(request: HttpRequest, _data: Data<()>, body: Simple<String>) -> Result<Json<Simple<String>>, MyError> {
    println!("echo - Request path: {:?}", request.path());
    println!("echo - Request method: {:?}", request.method());
    println!("echo - Request query_string: {:?}", request.query_string());
    println!("echo - Request body: {:?}", body);
    Ok(Json(body))
}

async fn info(request: HttpRequest, _data: Data<()>, body: InfoRequest<String>) -> Result<Json<InfoResponse<String>>, MyError> {
    println!("info - Request path: {:?}", request.path());
    println!("info - Request method: {:?}", request.method());
    println!("info - Request query_string: {:?}", request.query_string());
    println!("info - Request body: {:?}", body);

    let mut request_headers = HashMap::new();

    for (name, value) in request.headers() {
        request_headers.insert(name.to_string(), value.to_str().expect("Header value should be a string").to_owned());
    }

    Ok(Json(InfoResponse {
        request_headers,
        request_method: request.method().as_str().to_string(),
        request_path: request.path().to_string(),
        request_payload: body.payload,
        request_query_string: request.query_string().to_string(),
    }))
}

/// spanws an actix server and returns the server port
pub fn spawn_actix_web<REST: 'static + Clone + Send + RestType<Simple<String>, Simple<String>>>(echo_rest: REST) -> u16 {
    let free_port = port_check::free_local_port().unwrap();
    let address = format!("127.0.0.1:{}", free_port);

    // Start Server
    spawn(async move {
        println!("Start actix-web to {}", address);
        HttpServer::new(move || App::new()
                .app_data(Data::new(()))
                .service(echo_rest.handle(echo))
                .service(INFO_DELETE.handle(info))
                .service(INFO_GET.handle(info))
                .service(INFO_POST.handle(info))
                .service(INFO_PUT.handle(info))
            )
            .bind(&address)
            .and_then(|ser| Ok(ser))
            .unwrap()
            .run()
            .await
            .unwrap();
    });
    free_port
}