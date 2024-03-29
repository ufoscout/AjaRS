use std::time::Duration;

use ajars::surf::AjarsClientSurf;
use ajars::RestFluent;
use ajars_test::api::Simple;
use ajars_test::axum::spawn_axum;
use tokio::time::sleep;

#[actix_rt::test]
async fn test_surf_rest() {
    perform_surf_call(&RestFluent::<Simple<String>, Simple<String>>::delete(format!(
        "/api/{}",
        rand::random::<usize>()
    )))
    .await;
    perform_surf_call(&RestFluent::<Simple<String>, Simple<String>>::get(format!("/api/{}", rand::random::<usize>())))
        .await;
    perform_surf_call(&RestFluent::<Simple<String>, Simple<String>>::post(format!("/api/{}", rand::random::<usize>())))
        .await;
    perform_surf_call(&RestFluent::<Simple<String>, Simple<String>>::put(format!("/api/{}", rand::random::<usize>())))
        .await;
}

async fn perform_surf_call(rest: &RestFluent<Simple<String>, Simple<String>>) {
    // Arrange
    let rest_clone = rest.clone();
    let port = spawn_axum(rest_clone);
    sleep(Duration::from_millis(200)).await;

    // Start client
    let req = AjarsClientSurf::new(ajars::surf::surf::client(), format!("http://127.0.0.1:{}", port));

    let req_data = Simple { inner: format!("{}", rand::random::<usize>()) };

    // Act
    let response = req.request(rest).send(&req_data).await;

    // Assert
    assert_eq!(req_data, response.unwrap());
}
