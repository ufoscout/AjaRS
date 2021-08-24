use tokio::time::sleep;
use ajars_test::axum::spawn_axum;
use ajars_test::api::Simple;
use std::time::Duration;

use ajars::{Rest, RestFluent, RestType, actix_web::actix_web::http::header, reqwest::{reqwest::ClientBuilder, AjarsReqwest}};

#[actix_rt::test]
async fn test_reqwest_rest() {
    perform_reqwest_call(&RestFluent::<Simple<String>, Simple<String>>::delete(format!(
        "/api/{}",
        rand::random::<usize>()
    )))
    .await;
    perform_reqwest_call(&Rest::<Simple<String>, Simple<String>>::get("/api/const")).await;
    perform_reqwest_call(&RestFluent::<Simple<String>, Simple<String>>::post(format!(
        "/api/{}",
        rand::random::<usize>()
    )))
    .await;
    perform_reqwest_call(&RestFluent::<Simple<String>, Simple<String>>::put(format!(
        "/api/{}",
        rand::random::<usize>()
    )))
    .await;
}

async fn perform_reqwest_call<REST: 'static + Clone + Send + RestType<Simple<String>, Simple<String>>>(
    rest: &REST,
) {
    // Arrange
    let rest_clone = rest.clone();
    let port = spawn_axum(rest_clone);
    sleep(Duration::from_millis(200)).await;

    // Start client
    let ajars = AjarsReqwest::new(ClientBuilder::new().build().unwrap(), format!("http://127.0.0.1:{}", port));

    let req_data = Simple { inner: format!("{}", rand::random::<usize>()) };

    // Act
    let response = ajars
        .request(rest)
        .header(header::CONTENT_TYPE, "application/json")
        .send(&req_data).await;

    // Assert
    assert_eq!(req_data, response.unwrap());
}


