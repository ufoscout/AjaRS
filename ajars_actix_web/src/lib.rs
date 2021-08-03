use std::future::Future;

use ::actix_web::{FromRequest, Resource, ResponseError, web::{self, Json}};
use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};

pub mod actix_web {
    pub use actix_web::*;
}

mod attempt;

pub trait ActixWebHandler<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, T, H> {
    fn handle(&self, handler: H) -> Resource;
}


impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, H, R, E, REST: RestType<I, O>> ActixWebHandler<I, O, (), H>
    for REST 
where 
H: Clone + 'static + Fn(I) -> R,
R: Future<Output = Result<Json<O>, E>> + 'static,
E: ResponseError + 'static,
{
    fn handle(&self, handler: H) -> Resource {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => resource.route(web::delete().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
            HttpMethod::GET => resource.route(web::get().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
            HttpMethod::POST => resource.route(web::post().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
            HttpMethod::PUT => resource.route(web::put().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
        }
    }
}


impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, H, R, E, P0, REST: RestType<I, O>> ActixWebHandler<I, O, (P0,), H>
    for REST 
where 
H: Clone + 'static + Fn(I, P0) -> R,
R: Future<Output = Result<Json<O>, E>> + 'static,
E: ResponseError + 'static,
P0: FromRequest + 'static
{
    fn handle(&self, handler: H) -> Resource {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => resource.route(web::delete().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
            HttpMethod::GET => resource.route(web::get().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
            HttpMethod::POST => resource.route(web::post().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
            HttpMethod::PUT => resource.route(web::put().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;
    use crate::actix_web::test;
    use ::actix_web::{App, HttpRequest, web::Data};
    use ajars_core::RestFluent;
    use serde::{Deserialize, Serialize};
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {
        pub message: String,
    }
    
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {
        pub message: String,
    }

    async fn ping(body: PingRequest, _request: HttpRequest ) -> Result<Json<PingResponse>, ServerError> {
        Ok(Json(PingResponse { message: body.message }))
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
        let rest = RestFluent::<PingRequest, PingResponse>::delete(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(()))
                .service(rest.handle(ping)),
        )
        .await;

        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        let req = test::TestRequest::delete()
        .uri(&format!("{}?message={}", rest.path(), payload.message))
        .to_request();

        // Act
        let resp: PingResponse = test::read_response_json(&mut app, req).await;

        // Assert
        assert_eq!(resp.message, payload.message);
    }

    #[actix_rt::test]
    async fn should_create_a_get_endpoint() {
 
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::get(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(()))
                .service(rest.handle(ping)),
        )
        .await;

        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        let req = test::TestRequest::get()
        .uri(&format!("{}?message={}", rest.path(), payload.message))
        .to_request();

        // Act
        let resp: PingResponse = test::read_response_json(&mut app, req).await;

        // Assert
        assert_eq!(resp.message, payload.message);
    }

    #[actix_rt::test]
    async fn should_create_a_post_endpoint() {
 
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::post(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(()))
                .service(rest.handle(ping)),
        )
        .await;

        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        let req = test::TestRequest::post().uri(rest.path()).set_json(&payload).to_request();

        // Act
        let resp: PingResponse = test::read_response_json(&mut app, req).await;

        // Assert
        assert_eq!(resp.message, payload.message);
    }

    #[actix_rt::test]
    async fn should_create_a_put_endpoint() {
 
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::put(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(()))
                .service(rest.handle(ping)),
        )
        .await;

        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        let req = test::TestRequest::put().uri(rest.path()).set_json(&payload).to_request();

        // Act
        let resp: PingResponse = test::read_response_json(&mut app, req).await;

        // Assert
        assert_eq!(resp.message, payload.message);
    }

}