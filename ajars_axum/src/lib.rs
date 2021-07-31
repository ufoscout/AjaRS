use std::future::Future;
use ::axum::{
    extract::{Extension, RequestParts}, prelude::*, response::IntoResponse, routing::BoxRoute,
};
use ajars_core::{HttpMethod, RestType};
use http::Request;
use serde::{de::DeserializeOwned, Serialize};

pub mod axum {
    pub use axum::*;
}

pub trait AxumRoute<I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static> {

    /*
fn route<T, F, B>(&self, route: Route<T, F>, description: &str, svc: T) -> BoxRoute<Body>
where
    T: Service<Request<B>> + Clone;
    */

    fn route<D, E, F, R>(&self, handler: F) -> BoxRoute<Body>
where
    F: 'static + Clone + Send + Sync + Fn(
        //Request<Body>, 
        Extension<D>, I) -> R,
    R: Future<Output = Result<O, E>> + Send,
    D: Clone + Send + Sync + 'static,
    E: IntoResponse + Send + 'static;
}


impl<I: Serialize + DeserializeOwned + Send + 'static, O: Serialize + DeserializeOwned + Send + 'static, REST: RestType<I, O>>
AxumRoute<I, O> for REST
{
    fn route<D, E, F, R>(&self, handler: F) -> BoxRoute<Body>
where
    F: 'static + Clone + Send + Sync + Fn(
        //Request<Body>, 
        Extension<D>, I) -> R,
    R: Future<Output = Result<O, E>> + Send,
    D: Clone + Send + Sync + 'static,
    E: IntoResponse + Send + 'static
    {

        let route = match self.method() {
            HttpMethod::DELETE => route(self.path(), delete(|
                //req: Request<Body>, 
                data: Extension<D>, payload: extract::Query<I>| async move {
                let result = (handler)(data, payload.0).await;
                match result {
                    Ok(result) => Ok(response::Json(result)),
                    Err(e) => Err(e)
                }
            })).boxed(),
            HttpMethod::GET => route(self.path(), get(|
                //req: Request<Body>, 
                data: Extension<D>, payload: extract::Query<I>| async move {
                let result = (handler)(data, payload.0).await;
                match result {
                    Ok(result) => Ok(response::Json(result)),
                    Err(e) => Err(e)
                }
            })).boxed(),
            HttpMethod::POST => route(self.path(), post(|
                //req: Request<Body>, 
                data: Extension<D>, payload: extract::Json<I>| async move {
                let result = (handler)(data, payload.0).await;
                match result {
                    Ok(result) => Ok(response::Json(result)),
                    Err(e) => Err(e)
                }
            })).boxed(),
            HttpMethod::PUT => route(self.path(), put(|
                //req: Request<Body>, 
                data: Extension<D>, payload: extract::Json<I>| async move {
                let result = (handler)(data, payload.0).await;
                match result {
                    Ok(result) => Ok(response::Json(result)),
                    Err(e) => Err(e)
                }
            })).boxed(),
        };

        route
    }
}

#[cfg(test)]
mod tests {

    use std::fmt::Display;

    use super::*;
    use ajars_core::RestFluent;
    use ::axum::AddExtensionLayer;
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
        //_request: Request<Body>, 
        _data: Extension<()>, body: PingRequest) -> Result<PingResponse, ServerError> {
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
async fn test_extractors() {

    let app = route("/api", 
    post(|req: Request<Body>, payload: extract::Json<String>| async move {
        println!("CALL RECEIVED");
    }));

    // Act
    let response = app
        .oneshot(Request::builder()
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, "application/json")
            .uri("/api")
            .body(Body::from(
                serde_json::to_vec(&"payload").unwrap(),
            )).unwrap())
        .await
        .unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

}

}