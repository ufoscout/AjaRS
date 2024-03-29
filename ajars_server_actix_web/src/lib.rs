use std::future::Future;

use ::actix_web::web::{self, Json, Query};
use ::actix_web::{FromRequest, Resource, ResponseError};
use ajars_core::{HttpMethod, RestType};
use futures_util::future::FutureExt;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod actix_web {
    pub use ::actix_web::*;
}

pub trait AjarsServerActixWebHandler<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, T, H> {
    fn to(&self, handler: H) -> Resource;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    #[allow(non_snake_case)]
    impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, H, R, E, REST: RestType<I, O>, $($param,)*> AjarsServerActixWebHandler<I, O, ($($param,)*), H>
    for REST
where
H: Clone + 'static + Fn(I, $($param,)*) -> R,
R: Future<Output = Result<O, E>> + 'static,
E: ResponseError + 'static,
$( $param: FromRequest + 'static, )*
{
    fn to(&self, handler: H) -> Resource {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => resource.route(web::delete().to(
                move |json: Query<I>, $( $param: $param,)*| {
                (handler)(json.into_inner(), $($param,)*).map(|res| res.map(Json))
            })),
            HttpMethod::GET => resource.route(web::get().to(
                move |json: Query<I>, $( $param: $param,)*| {
                (handler)(json.into_inner(), $($param,)*).map(|res| res.map(Json))
            })),
            HttpMethod::POST => resource.route(web::post().to(
                move |json: Json<I>, $( $param: $param,)*| {
                (handler)(json.into_inner(), $($param,)*).map(|res| res.map(Json))
            })),
            HttpMethod::PUT => resource.route(web::put().to(
                move |json: Json<I>, $( $param: $param,)*| {
                (handler)(json.into_inner(), $($param,)*).map(|res| res.map(Json))
            })),
        }
    }
}
});

factory_tuple! {}
factory_tuple! { P0 }
factory_tuple! { P0 P1 }
factory_tuple! { P0 P1 P2 }
factory_tuple! { P0 P1 P2 P3 }
factory_tuple! { P0 P1 P2 P3 P4 }
factory_tuple! { P0 P1 P2 P3 P4 P5 }
factory_tuple! { P0 P1 P2 P3 P4 P5 P6 }
factory_tuple! { P0 P1 P2 P3 P4 P5 P6 P7 }
factory_tuple! { P0 P1 P2 P3 P4 P5 P6 P7 P8 }
//factory_tuple! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 }

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use ::actix_web::http::{header, StatusCode};
    use ::actix_web::{App, HttpRequest};
    use ajars_core::RestFluent;
    use serde::{Deserialize, Serialize};

    use super::*;
    use crate::actix_web::dev::Service;
    use crate::actix_web::test;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {
        pub message: String,
    }

    async fn ping(body: PingRequest, _request: HttpRequest) -> Result<PingResponse, ServerError> {
        Ok(PingResponse { message: body.message })
    }

    #[derive(Debug, Clone)]
    struct ServerError {}

    impl Display for ServerError {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl ResponseError for ServerError {}

    #[actix_rt::test]
    async fn should_create_a_delete_endpoint() {
        // Arrange
        let rest =
            RestFluent::<PingRequest, PingResponse>::delete(format!("/api/something/{}", rand::random::<usize>()));

        let app = test::init_service(App::new().service(rest.to(ping))).await;

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        let req = test::TestRequest::delete().uri(&format!("{}?message={}", rest.path(), payload.message)).to_request();

        // Act
        let resp = app.call(req).await.unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!("application/json", resp.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let resp: PingResponse = test::read_body_json(resp).await;
        assert_eq!(resp.message, payload.message);
    }

    #[actix_rt::test]
    async fn should_create_a_get_endpoint() {
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::get(format!("/api/something/{}", rand::random::<usize>()));

        let app = test::init_service(App::new().service(rest.to(ping))).await;

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        let req = test::TestRequest::get().uri(&format!("{}?message={}", rest.path(), payload.message)).to_request();

        // Act
        let resp = app.call(req).await.unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!("application/json", resp.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let resp: PingResponse = test::read_body_json(resp).await;
        assert_eq!(resp.message, payload.message);
    }

    #[actix_rt::test]
    async fn should_create_a_post_endpoint() {
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::post(format!("/api/something/{}", rand::random::<usize>()));

        let app = test::init_service(App::new().service(rest.to(ping))).await;

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        let req = test::TestRequest::post().uri(rest.path()).set_json(&payload).to_request();

        // Act
        let resp = app.call(req).await.unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!("application/json", resp.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let resp: PingResponse = test::read_body_json(resp).await;
        assert_eq!(resp.message, payload.message);
    }

    #[actix_rt::test]
    async fn should_create_a_put_endpoint() {
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::put(format!("/api/something/{}", rand::random::<usize>()));

        let app = test::init_service(App::new().service(rest.to(ping))).await;

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        let req = test::TestRequest::put().uri(rest.path()).set_json(&payload).to_request();

        // Act
        let resp = app.call(req).await.unwrap();

        // Assert
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!("application/json", resp.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let resp: PingResponse = test::read_body_json(resp).await;
        assert_eq!(resp.message, payload.message);
    }
}
