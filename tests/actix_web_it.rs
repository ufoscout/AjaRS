#![cfg(feature = "server_actix_web")]

use actix_rt::spawn;
use actix_rt::time::sleep;
use actix_web::web::{Data, Json};
use actix_web::{App, HttpRequest, HttpServer, ResponseError};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::Duration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
struct Simple<O> {
    pub inner: O,
}

#[derive(Debug, Clone)]
struct MyError {}

impl Display for MyError {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl ResponseError for MyError {}

async fn echo(
    request: HttpRequest,
    _data: Data<()>,
    body: Simple<String>,
) -> Result<Json<Simple<String>>, MyError> {
    println!("echo - Request path: {:?}", request.path());
    println!("echo - Request method: {:?}", request.method());
    println!("echo - Request query_string: {:?}", request.query_string());
    println!("echo - Request body: {:?}", body);
    Ok(Json(body))
}

#[cfg(feature = "client_reqwest")]
mod actix_web_reqwest_it {

    use super::*;
    use actix_web::web::Data;
    use ajars::{reqwest::RestReqwest, Rest};

    #[actix_rt::test]
    async fn test_reqwest_rest() {
        perform_reqwest_call(&Rest::<Simple<String>, Simple<String>>::delete(format!(
            "/api/{}",
            rand::random::<usize>()
        )))
        .await;
        perform_reqwest_call(&Rest::<Simple<String>, Simple<String>>::get(format!(
            "/api/{}",
            rand::random::<usize>()
        )))
        .await;
        perform_reqwest_call(&Rest::<Simple<String>, Simple<String>>::post(format!(
            "/api/{}",
            rand::random::<usize>()
        )))
        .await;
        perform_reqwest_call(&Rest::<Simple<String>, Simple<String>>::put(format!(
            "/api/{}",
            rand::random::<usize>()
        )))
        .await;
    }

    async fn perform_reqwest_call(rest: &Rest<Simple<String>, Simple<String>>) {
        // Arrange
        let free_port = port_check::free_local_port().unwrap();
        let address = format!("127.0.0.1:{}", free_port);

        // Start Server
        let address_clone = address.clone();
        let rest_clone = rest.clone();

        spawn(async move {
            println!("Start actix-web to {}", address_clone);
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(()))
                    .service(rest_clone.handle(echo))
            })
            .bind(&address_clone)
            .and_then(|ser| Ok(ser))
            .unwrap()
            .run()
            .await
            .unwrap();
        });

        sleep(Duration::from_millis(200)).await;

        // Start client
        let req = RestReqwest::new(
            reqwest::ClientBuilder::new().build().unwrap(),
            format!("http://{}", address),
        );

        let req_data = Simple {
            inner: format!("{}", rand::random::<usize>()),
        };

        // Act
        let response = req.submit(&rest, &req_data).await;

        // Assert
        assert_eq!(req_data, response.unwrap());
    }
}
