use std::future::Future;
use ::axum::{extract::FromRequest, prelude::*, response::IntoResponse, routing::BoxRoute};
use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};

pub mod axum {
    pub use axum::*;
}

pub trait AxumHandler<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, T> {
    fn route<REST: RestType<I, O>>(self, rest: &REST) -> BoxRoute<Body>;
}


impl <I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static, F, R, E> AxumHandler<I, O, ()> for F
where 
R: Future<Output = Result<O, E>> + Send,
E: IntoResponse + Send + 'static,
F: 'static + Clone + Send + Sync + Fn(I) -> R,
{
    fn route<REST: RestType<I, O>>(self, rest: &REST) -> BoxRoute<Body> {
        let route = match rest.method() {
            HttpMethod::DELETE => route(rest.path(), delete(
                |payload: extract::Query<I>| async move {
                (self)(payload.0).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::GET => route(rest.path(), get(
                |payload: extract::Query<I>| async move {
                (self)(payload.0).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::POST => route(rest.path(), post(
                |payload: extract::Json<I>| async move {
                (self)(payload.0).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::PUT => route(rest.path(), put(
                |payload: extract::Json<I>| async move {
                (self)(payload.0).await.map(|result| response::Json(result))
            })).boxed(),
        };

        route
    }
}


impl <I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static, F, R, E, P1> AxumHandler<I, O, (P1,)> for F
where 
R: Future<Output = Result<O, E>> + Send,
E: IntoResponse + Send + 'static,
F: 'static + Send + Sync + Clone + Fn(I, P1) -> R,
P1: FromRequest<Body> + Send + 'static
{
    fn route<REST: RestType<I, O>>(self, rest: &REST) -> BoxRoute<Body> {
        let route = match rest.method() {
            HttpMethod::DELETE => route(rest.path(), delete(
                |payload: extract::Query<I>, p1: P1| async move {
                (self)(payload.0, p1).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::GET => route(rest.path(), get(
                |payload: extract::Query<I>, p1: P1| async move {
                (self)(payload.0, p1).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::POST => route(rest.path(), post(
                |payload: extract::Json<I>, p1: P1| async move {
                (self)(payload.0, p1).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::PUT => route(rest.path(), put(
                |payload: extract::Json<I>, p1: P1| async move {
                (self)(payload.0, p1).await.map(|result| response::Json(result))
            })).boxed(),
        };

        route
    }
}

impl <I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static, F, R, E, P1, P2> AxumHandler<I, O, (P1, P2)> for F
where 
R: Future<Output = Result<O, E>> + Send,
E: IntoResponse + Send + 'static,
F: 'static + Send + Sync + Clone + Fn(I, P1, P2) -> R,
P1: FromRequest<Body> + Send + 'static,
P2: FromRequest<Body> + Send + 'static,
{
    fn route<REST: RestType<I, O>>(self, rest: &REST) -> BoxRoute<Body> {
        let route = match rest.method() {
            HttpMethod::DELETE => route(rest.path(), delete(
                |payload: extract::Query<I>, p1: P1, p2: P2| async move {
                (self)(payload.0, p1, p2).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::GET => route(rest.path(), get(
                |payload: extract::Query<I>, p1: P1, p2: P2| async move {
                (self)(payload.0, p1, p2).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::POST => route(rest.path(), post(
                |payload: extract::Json<I>, p1: P1, p2: P2| async move {
                (self)(payload.0, p1, p2).await.map(|result| response::Json(result))
            })).boxed(),
            HttpMethod::PUT => route(rest.path(), put(
                |payload: extract::Json<I>, p1: P1, p2: P2| async move {
                (self)(payload.0, p1, p2).await.map(|result| response::Json(result))
            })).boxed(),
        };

        route
    }
}

pub trait AxumRoute<I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static> {
    fn route<T, H: AxumHandler<I, O, T>>(&self, handler: H) -> BoxRoute<Body>;
}

impl <I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static, REST: RestType<I, O>>
AxumRoute<I, O> for REST {

    fn route<T, H: AxumHandler<I, O, T>>(&self, handler: H) -> BoxRoute<Body> {
        handler.route(self)
    }

} 


#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;
    use ajars_core::RestFluent;
    use ::axum::{AddExtensionLayer, extract::Extension};
    use http::StatusCode;
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

    async fn ping(
        body: PingRequest, _data: Extension<()>) -> Result<PingResponse, ServerError> {
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
        fn into_response(self) -> http::Response<Body> {
            "error".into_response()
        }
    }

    #[tokio::test]
    async fn should_create_a_delete_endpoint() {

        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::delete(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let app = rest
            .route(ping)
            .layer(AddExtensionLayer::new(()));


        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        // Act
        let response = app
            .oneshot(Request::builder()
                .method(http::Method::DELETE)
                .header(http::header::CONTENT_TYPE, "application/json")
                .uri(&format!("{}?message={}", rest.path(), payload.message))
                .body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn should_create_a_get_endpoint() {
 
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::get(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let app = rest
            .route(ping)
            .layer(AddExtensionLayer::new(()));


        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        // Act
        let response = app
            .oneshot(Request::builder()
                .method(http::Method::GET)
                .header(http::header::CONTENT_TYPE, "application/json")
                .uri(&format!("{}?message={}", rest.path(), payload.message))
                .body(Body::empty()).unwrap())
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn should_create_a_post_endpoint() {

        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::post(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));


        let app = rest
            .route(ping)
            .layer(AddExtensionLayer::new(()));

        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        // Act
        let response = app
            .oneshot(Request::builder()
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, "application/json")
                .uri(rest.path())
                .body(Body::from(
                    serde_json::to_vec(&payload).unwrap(),
                )).unwrap())
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }


    #[tokio::test]
    async fn should_create_a_put_endpoint() {
 
        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::put(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));

        let app = rest
            .route(ping)
            .layer(AddExtensionLayer::new(()));

        let payload = PingRequest {
            message: format!("message{}", rand::random::<usize>())
        };

        // Act
        let response = app
            .oneshot(Request::builder()
                .method(http::Method::PUT)
                .header(http::header::CONTENT_TYPE, "application/json")
                .uri(rest.path())
                .body(Body::from(
                    serde_json::to_vec(&payload).unwrap(),
                )).unwrap())
            .await
            .unwrap();

        // Assert
        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body: PingResponse = serde_json::from_slice(&body).unwrap();

        // Assert
        assert_eq!(body.message, payload.message);
    }

    #[tokio::test]
    async fn route_should_accept_variable_number_of_params() {

        // Arrange
        let rest = RestFluent::<PingRequest, PingResponse>::delete(format!(
            "/api/something/{}",
            rand::random::<usize>()
        ));

        // Accept 1 param
        rest.route(|body: PingRequest| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });

        // Accept 2 param
        rest.route(|body: PingRequest, _: Extension<()>| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });
        
        // Accept 3 param
        rest.route(|body: PingRequest, _: Extension<()>, _: Request<Body>| async {
            Result::<_, ServerError>::Ok(PingResponse { message: body.message })
        });
    }

}