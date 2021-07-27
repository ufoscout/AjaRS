use std::{future::Future, marker::PhantomData};

use crate::actix_web::{
    dev::Handler as ActixHandler,
    web::{Data, Json, Query},
    *,
};
use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};

pub mod actix_web {
    pub use actix_web::*;
}

pub trait HandleActix<I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static> {
    fn handle<H, D, R, E>(&self, handler: H) -> Resource
    where
        H: Handler<D, R, E, I, O>,
        D: 'static,
        R: Future<Output = Result<Json<O>, E>> + 'static,
        E: ResponseError + 'static;
}

impl<I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, REST: RestType<I, O>>
    HandleActix<I, O> for REST
{
    fn handle<H, D, R, E>(&self, handler: H) -> Resource
    where
        H: Handler<D, R, E, I, O>,
        D: 'static,
        R: Future<Output = Result<Json<O>, E>> + 'static,
        E: ResponseError + 'static,
    {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => resource.route(web::delete().to(QueryHandlerWrapper::new(handler))),
            HttpMethod::GET => resource.route(web::get().to(QueryHandlerWrapper::new(handler))),
            HttpMethod::POST => resource.route(web::post().to(JsonHandlerWrapper::new(handler))),
            HttpMethod::PUT => resource.route(web::put().to(JsonHandlerWrapper::new(handler))),
        }
    }
}

pub trait Handler<D, R, E, I, O>: Clone + 'static
where
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, I)) -> R;
}

impl<F, D, R, E, I, O> Handler<D, R, E, I, O> for F
where
    F: 'static + Clone + Fn(HttpRequest, Data<D>, I) -> R,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, I)) -> R {
        self(param.0, param.1, param.2)
    }
}

pub struct JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    handler: H,
    phantom_d: PhantomData<D>,
    phantom_r: PhantomData<R>,
    phantom_e: PhantomData<E>,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<H, D, R, E, I, O> JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            phantom_d: PhantomData,
            phantom_r: PhantomData,
            phantom_e: PhantomData,
            phantom_i: PhantomData,
            phantom_o: PhantomData,
        }
    }
}

impl<H, D, R, E, I, O> Clone for JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn clone(&self) -> Self {
        Self::new(self.handler.clone())
    }
}

impl<H, D, R, E, I, O> ActixHandler<(HttpRequest, Data<D>, Json<I>), R> for JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    D: 'static,
    R: Future<Output = Result<Json<O>, E>> + 'static,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, Json<I>)) -> R {
        self.handler.call((param.0, param.1, param.2.into_inner()))
    }
}

pub struct QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    handler: H,
    phantom_d: PhantomData<D>,
    phantom_r: PhantomData<R>,
    phantom_e: PhantomData<E>,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<H, D, R, E, I, O> QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    pub fn new(handler: H) -> Self {
        Self {
            handler,
            phantom_d: PhantomData,
            phantom_r: PhantomData,
            phantom_e: PhantomData,
            phantom_i: PhantomData,
            phantom_o: PhantomData,
        }
    }
}

impl<H, D, R, E, I, O> Clone for QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn clone(&self) -> Self {
        Self::new(self.handler.clone())
    }
}

impl<H, D, R, E, I, O> ActixHandler<(HttpRequest, Data<D>, Query<I>), R> for QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    D: 'static,
    R: Future<Output = Result<Json<O>, E>> + 'static,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, Query<I>)) -> R {
        self.handler.call((param.0, param.1, param.2.into_inner()))
    }
}

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;
    use crate::actix_web::test;
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

    async fn ping(_request: HttpRequest, _data: Data<()>, body: PingRequest) -> Result<Json<PingResponse>, ServerError> {
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