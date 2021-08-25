use ::axum::{
    extract::{self, FromRequest},
    handler::{delete, get, post, put},
    response::{self, IntoResponse},
    routing::BoxRoute,
    Router,
};
use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};
use std::future::Future;

pub mod axum {
    pub use axum::*;
}

pub trait AxumHandler<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, T, H> {
    fn to(&self, handler: H) -> Router<BoxRoute>;
}

macro_rules! factory_tuple ({ $($param:ident)* } => {
    #[allow(non_snake_case)]
    impl <I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static, H, R, E, REST: RestType<I, O>, $($param,)*> AxumHandler<I, O, ($($param,)*), H> 
    for REST
    where
    R: Future<Output = Result<O, E>> + Send,
    E: IntoResponse + Send + 'static,
    H: 'static + Send + Sync + Clone + Fn(I, $($param,)*) -> R,
    $( $param: FromRequest + Send + 'static, )*
    {
        fn to(&self, handler: H) -> Router<BoxRoute> {
            let route = match self.method() {
                HttpMethod::DELETE => Router::new().route(self.path(), delete(
                    |payload: extract::Query<I>, $( $param: $param,)*| async move {
                        (handler)(payload.0, $( $param,)*).await.map(response::Json)
                })).boxed(),
                HttpMethod::GET => Router::new().route(self.path(), get(
                    |payload: extract::Query<I>, $( $param: $param,)*| async move {
                        (handler)(payload.0, $( $param,)*).await.map(response::Json)
                    })).boxed(),
                HttpMethod::POST => Router::new().route(self.path(), post(
                    |payload: extract::Json<I>, $( $param: $param,)*| async move {
                        (handler)(payload.0, $( $param,)*).await.map(response::Json)
                    })).boxed(),
                HttpMethod::PUT => Router::new().route(self.path(), put(
                    |payload: extract::Json<I>, $( $param: $param,)*| async move {
                        (handler)(payload.0, $( $param,)*).await.map(response::Json)
                    })).boxed(),
            };

            route
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
factory_tuple! { P0 P1 P2 P3 P4 P5 P6 P7 P8 P9 }


#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;
    use ::axum::{
        body::Body,
        extract::Extension,
        http::{header, Method, Request, Response, StatusCode},
        AddExtensionLayer,
    };
    use ajars_core::RestFluent;
    use serde::{Deserialize, Serialize};
    use tower::ServiceExt; // for `app.oneshot()`

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {
        pub message: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {
        pub message: String,
    }

    async fn ping(body: PingRequest, _data: Extension<()>) -> Result<PingResponse, ServerError> {
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
        type Body = axum::body::Body;
        type BodyError = <Self::Body as axum::body::HttpBody>::Error;

        fn into_response(self) -> Response<Self::Body> {
            Response::new(Body::empty())
        }
    }

    #[tokio::test]
    async fn should_create_a_delete_endpoint() {
        // Arrange
        let rest =
            RestFluent::<PingRequest, PingResponse>::delete(format!("/api/something/{}", rand::random::<usize>()));

        let app = rest.to(ping).layer(AddExtensionLayer::new(()));

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

        let app = rest.to(ping).layer(AddExtensionLayer::new(()));

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

        let app = rest.to(ping).layer(AddExtensionLayer::new(()));

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

        let app = rest.to(ping).layer(AddExtensionLayer::new(()));

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
        rest.to(|body: PingRequest| async { Result::<_, ServerError>::Ok(PingResponse { message: body.message }) });

        // Accept 2 param
        rest.to(|body: PingRequest, _: Extension<()>| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 3 param
        rest.to(|body: PingRequest, _: Extension<()>, _: Request<Body>| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 4 param
        rest.to(|body: PingRequest, _: Extension<()>, _: Request<Body>, _: Request<Body>| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 5 param
        rest.to(|body: PingRequest, _: Extension<()>, _: Request<Body>, _: Request<Body>, _: Request<Body>| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });
    }
}
