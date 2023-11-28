use std::time::Duration;

use actix_rt::time::sleep;
use ajars::reqwest::reqwest::ClientBuilder;
use ajars::reqwest::AjarsClientReqwest;
use ajars::{Rest, RestFluent, RestType};
use ajars_test::actix_web::spawn_actix_web;
use ajars_test::api::Simple;

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

async fn perform_reqwest_call<REST: 'static + Clone + Send + RestType<Simple<String>, Simple<String>>>(rest: &REST) {
    // Arrange
    let rest_clone = rest.clone();
    let port = spawn_actix_web(rest_clone);
    sleep(Duration::from_millis(200)).await;

    // Start client
    let ajars = AjarsClientReqwest::new(ClientBuilder::new().build().unwrap(), format!("http://127.0.0.1:{}", port));

    let req_data = Simple { inner: format!("{}", rand::random::<usize>()) };

    // Act
    let response = ajars.request(rest).send(&req_data).await;

    // Assert
    assert_eq!(req_data, response.unwrap());
}
