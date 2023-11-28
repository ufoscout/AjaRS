use std::future::Future;

use ::axum::body::Body;
use ::axum::extract::{self, FromRequestParts};
use ::axum::response::IntoResponse;
use ::axum::routing::{delete, get, post, put};
use ::axum::{Json, Router};
use ajars_core::{HttpMethod, RestType};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub mod axum {
    pub use ::axum::*;
}

pub trait AxumHandler<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, T, H, S> {
    fn to(&self, handler: H) -> Router<S, Body>;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    #[allow(non_snake_case)]
    impl <I, O, H, R, E, S, REST: RestType<I, O>, $($param,)*> AxumHandler<I, O, ($($param,)*), H, S>
    for REST
    where
    I: Serialize + DeserializeOwned + Send + 'static,
    O: Serialize + DeserializeOwned + Send + 'static,
    R: Future<Output = Result<O, E>> + Send,
    E: IntoResponse + Send + 'static,
    S: Clone + Send + Sync + 'static,
    H: 'static + Send + Sync + Clone + Fn($($param,)* I) -> R,
    $( $param: FromRequestParts<S> + Send + 'static, )*
    {
        fn to(&self, handler: H) -> Router<S, Body> {
            let route = match self.method() {
                HttpMethod::DELETE => Router::new().route(self.path(), delete(
                    |$( $param: $param,)* payload: extract::Query<I>| async move {
                        (handler)($( $param,)* payload.0).await.map(Json)
                })),
                HttpMethod::GET => Router::new().route(self.path(), get(
                    |$( $param: $param,)* payload: extract::Query<I>| async move {
                        (handler)($( $param,)* payload.0).await.map(Json)
                    })),
                HttpMethod::POST => Router::new().route(self.path(), post(
                    |$( $param: $param,)* payload: Json<I>| async move {
                        (handler)($( $param,)* payload.0).await.map(Json)
                    })),
                HttpMethod::PUT => Router::new().route(self.path(), put(
                    |$( $param: $param,)* payload: Json<I>| async move {
                        (handler)($( $param,)* payload.0).await.map(Json)
                    })),
            };

            route
        }
    }
});

//
// MODEL FN USED FOR CREATING THE MACRO
//
// impl <I, O, H, R, E, S, REST: RestType<I, O>, P> AxumHandler<I, O, P, H, S>
// for REST
// where
// I: Serialize + DeserializeOwned + Send + 'static,
// O: Serialize + DeserializeOwned + Send + 'static,
// R: Future<Output = Result<O, E>> + Send,
// E: IntoResponse + Send + 'static,
// H: 'static + Send + Sync + Clone + Fn(P, I) -> R,
// S: Clone + Send + Sync + 'static,
// P: FromRequestParts<S> + Send + 'static,
// {
//     fn to(&self, handler: H) -> Router<S, Body> {
//         let route = match self.method() {
//             HttpMethod::DELETE => Router::new().route(self.path(), delete(
//                 |p: P, payload: extract::Query<I>| async move {
//                     (handler)(p, payload.0).await.map(Json)
//             })),
//             HttpMethod::GET => Router::new().route(self.path(), get(
//                 |p: P, payload: extract::Query<I>| async move {
//                     (handler)(p, payload.0).await.map(Json)
//                 })),
//             HttpMethod::POST => Router::new().route(self.path(), post(
//                 |p: P, payload: extract::Json<I>| async move {
//                     (handler)(p, payload.0).await.map(Json)
//                 })),
//             HttpMethod::PUT => Router::new().route(self.path(), put(
//                 |p: P, payload: extract::Json<I>| async move {
//                     (handler)(p, payload.0).await.map(Json)
//                 })),
//         };
//         route
//     }
// }

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
factory_tuple! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 }

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use ::axum::body::{Body, BoxBody};
    use ::axum::extract::{Extension, Query, State};
    use ::axum::http::{header, Method, Request, Response, StatusCode};
    use ajars_core::RestFluent;
    use serde::{Deserialize, Serialize};
    use tower::ServiceExt;

    use super::*; // for `app.oneshot()`

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {
        pub message: String,
    }

    async fn ping(_data: State<()>, body: PingRequest) -> Result<PingResponse, ServerError> {
        Ok(PingResponse { message: body.message })
    }

    #[derive(Debug, Clone)]
    struct ServerError {}

    impl Display for ServerError {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }

    impl IntoResponse for ServerError {
        fn into_response(self) -> Response<BoxBody> {
            Response::new(axum::body::boxed(Body::empty()))
        }
    }

    #[tokio::test]
    async fn should_create_a_delete_endpoint() {
        // Arrange
        let rest =
            RestFluent::<PingRequest, PingResponse>::delete(format!("/api/something/{}", rand::random::<usize>()));

        let app = rest.to(ping).with_state(());

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::DELETE)
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(&format!("{}?message={}", rest.path(), payload.message))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!("application/json", response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn should_create_a_get_endpoint() {
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::get(format!("/api/something/{}", rand::random::<usize>()));

        let app = rest.to(ping).layer(Extension(()));

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::GET)
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(&format!("{}?message={}", rest.path(), payload.message))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!("application/json", response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn should_create_a_post_endpoint() {
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::post(format!("/api/something/{}", rand::random::<usize>()));

        let app = rest.to(ping).layer(Extension(()));

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(rest.path())
                    .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!("application/json", response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn should_create_a_put_endpoint() {
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::put(format!("/api/something/{}", rand::random::<usize>()));

        let app = rest.to(ping).layer(Extension(()));

        let payload = PingRequest { message: format!("message{}", rand::random::<usize>()) };

        // Act
        let response = app
            .oneshot(
                Request::builder()
                    .method(Method::PUT)
                    .header(header::CONTENT_TYPE, "application/json")
                    .uri(rest.path())
                    .body(Body::from(serde_json::to_vec(&payload).unwrap()))
                    .unwrap(),
            )
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!("application/json", response.headers().get(header::CONTENT_TYPE).unwrap().to_str().unwrap());

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn route_should_accept_variable_number_of_params() {
        // Arrange
        let rest =
            RestFluent::<PingRequest, PingResponse>::delete(format!("/api/something/{}", rand::random::<usize>()));

        // Accept 1 param
        let _ = rest
            .to(|body: PingRequest| async { Result::<_, ServerError>::Ok(PingResponse { message: body.message }) })
            .with_state::<()>(());

        // Accept 2 param
        let _ = rest.to(|_: State<()>, body: PingRequest| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 3 param
        let _ = rest.to(|_: State<String>, _: Query<String>, body: PingRequest| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 4 param
        let _ = rest.to(|_: State<()>, _: Query<String>, _: Query<usize>, body: PingRequest| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 5 param
        let _ = rest.to(|_: State<()>, _: Query<String>, _: Query<u64>, _: Query<()>, body: PingRequest| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });
    }
}
