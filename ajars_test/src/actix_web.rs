use std::collections::HashMap;

use actix_rt::spawn;
use ajars::{
    actix_web::{
        actix_web::{web::Data, App, HttpRequest, HttpServer, ResponseError},
        ActixWebHandler,
    },
    RestType,
};

use crate::{api::*, error::MyError};

impl ResponseError for MyError {}

async fn echo(body: Simple<String>, request: HttpRequest, _data: Data<()>) -> Result<Simple<String>, MyError> {
    println!("echo - Request path: {:?}", request.path());
    println!("echo - Request method: {:?}", request.method());
    println!("echo - Request query_string: {:?}", request.query_string());
    println!("echo - Request body: {:?}", body);
    Ok(body)
}

async fn info(
    body: InfoRequest<String>,
    request: HttpRequest,
    _data: Data<()>,
) -> Result<InfoResponse<String>, MyError> {
    println!("info - Request path: {:?}", request.path());
    println!("info - Request method: {:?}", request.method());
    println!("info - Request query_string: {:?}", request.query_string());
    println!("info - Request body: {:?}", body);

    let mut request_headers = HashMap::new();

    for (name, value) in request.headers() {
        request_headers.insert(name.to_string(), value.to_str().expect("Header value should be a string").to_owned());
    }

    Ok(InfoResponse {
        request_headers,
        request_method: request.method().as_str().to_string(),
        request_path: request.path().to_string(),
        request_payload: body.payload,
        request_query_string: request.query_string().to_string(),
    })
}

/// spanws an actix server and returns the server port
pub fn spawn_actix_web<REST: 'static + Clone + Send + RestType<Simple<String>, Simple<String>>>(
    echo_rest: REST,
) -> u16 {
    let free_port = port_check::free_local_port().unwrap();
    let address = format!("127.0.0.1:{}", free_port);

    // Start Server
    spawn(async move {
        println!("Start actix-web to {}", address);
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(()))
                .service(echo_rest.to(echo))
                .service(INFO_DELETE.to(info))
                .service(INFO_GET.to(info))
                .service(INFO_POST.to(info))
                .service(INFO_PUT.to(info))
        })
        .bind(&address)
        .unwrap()
        .run()
        .await
        .unwrap();
    });
    free_port
}
