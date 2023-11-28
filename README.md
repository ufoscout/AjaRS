# AjaRS

![crates.io](https://img.shields.io/crates/v/ajars.svg)
![Build Status](https://github.com/ufoscout/ajars/actions/workflows/build_and_test.yml/badge.svg)
[![codecov](https://codecov.io/gh/ufoscout/ajars/branch/master/graph/badge.svg)](https://codecov.io/gh/ufoscout/ajars)


A small [Rust](https://www.rust-lang.org) library to remove the duplicated code between the definition of a Server side REST endpoint 
and the one of a REST Client that calls it.


## The problem
When we create a REST endpoint, we need to provide at least four different values:
1. The path of the resource
1. The HTTP Method
1. The JSON type consumed
1. The JSON type produced

Exactly the same four values have to be provided when creating a REST client for that endpoint.

For example, if we use [actix-web](https://github.com/actix/actix-web), an endpoint could be created with:
```rust
#[cfg(all(feature = "actix_web", feature = "reqwest"))]
mod without_ajars {
    use ajars::actix_web::actix_web::{App, Result, web::{self, Json}};
    use serde::{Deserialize, Serialize};

    fn server() {
        App::new().service(

            web::resource("/ping")  // PATH definition here
            
            .route(web::post()      // HTTP Method definition here
            
            .to(ping)               // The signature of the `ping` fn determines the
                                    // JSON types produced and consumed. In this case
                                    // PingRequest and PingResponse
            )
        );

        async fn ping(_body: Json<PingRequest>) -> Result<Json<PingResponse>> {
            Ok(Json(PingResponse {}))
        }
    }

    // Let's now declare a client using [reqwest](https://github.com/seanmonstar/reqwest)
    pub async fn client() {
        
        use ajars::reqwest::reqwest::ClientBuilder;
        
        let client = ClientBuilder::new().build().unwrap();

        let url = "http://127.0.0.1:8080/ping";    // Duplicated '/ping' path definition
        
        client.post(url)                           // Duplicated HTTP Post method definition
        
        .json(&PingRequest {})                     // Duplicated request type. Not checked at compile time
        
        .send().await.unwrap()
        .json::<PingResponse>().await.unwrap();    // Duplicated response type. Not checked at compile time
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```

Wouldn't it be good to have those values declared only once with all types checked at compile time?


## The AjaRs solution

Ajars allows a single definition for both the client and server. This removes code duplication and, at the same time,
allows compile time verification that the request and response types are correct.

Let's now redefine the previous endpoint using Ajars: definition of the previous endpoint:
```rust
#[cfg(all(feature = "actix_web", feature = "reqwest"))]
mod with_ajars {
    use ajars::Rest;
    use serde::{Deserialize, Serialize};

    // This defines a 'POST' call with path /ping, request type 'PingRequest' and response type 'PingResponse'
    // This should ideally be declared in a commond library imported by both the server and the client
    pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");


    // The the server side endpoint creation now becomes:
    fn server() {

        use ajars::actix_web::AjarsServerActixWebHandler;
        use ajars::{actix_web::actix_web::{App, HttpServer, ResponseError}};
        use derive_more::{Display, Error};

        HttpServer::new(move || 
            App::new().service(
                PING.to(ping) // here Ajarj takes care of the endpoint creation
            )
        );

        #[derive(Debug, Display, Error)]
        enum UserError {
            #[display(fmt = "Validation error on field: {}", field)]
            ValidationError { field: String },
        }
        impl ResponseError for UserError {}

        async fn ping(_body: PingRequest) -> Result<PingResponse, UserError> {
            Ok(PingResponse {})
        }

        // start the server...

    }
        
    // The client, using reqwest, becomes:
    async fn client() {
        
        use ajars::reqwest::{AjarsClientReqwest, reqwest::ClientBuilder};

        let ajars = AjarsClientReqwest::new(ClientBuilder::new().build().unwrap(), "http://127.0.0.1:8080");
        
        // Performs a POST request to http://127.0.0.1:8080/ping
        // The PingRequest and PingResponse types are enforced at compile time
        let response = ajars
            .request(&PING)
            .send(&PingRequest {})
            .await
            .unwrap();
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```

## Supported clients

### WASM (web-sys) in the browser
Ajars provides a lightweight client implementation based on [web-sys](https://github.com/rustwasm/wasm-bindgen), 
this is to be used in WASM based web frontends that run in a browser (e.g. [Yew](https://github.com/yewstack/yew), 
[Sycamore](https://github.com/sycamore-rs/sycamore), etc...).

To use it enable the `web` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["web"] }
```

Example:
```rust
#[cfg(feature = "web")]
mod web { 
    use ajars::web::AjarsClientWeb;
    use ajars::Rest;
    use serde::{Deserialize, Serialize};

    pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");

    async fn client() {

        let ajars = AjarsClientWeb::new("").expect("Should build Ajars");
        
        let response = ajars
            .request(&PING)           // <-- Here's everything required
            .send(&PingRequest {})
            .await
            .unwrap();
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```

### Reqwest
To use it with [reqwest](https://github.com/seanmonstar/reqwest) enable the `reqwest` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["reqwest"] }
```

Example:
```rust
#[cfg(feature = "reqwest")]
mod reqwest { 
    use ajars::Rest;
    use ajars::reqwest::{AjarsClientReqwest, reqwest::ClientBuilder};
    use serde::{Deserialize, Serialize};

    pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");

    async fn client() {

        let ajars = AjarsClientReqwest::new(ClientBuilder::new().build().unwrap(), "http://127.0.0.1:8080");
        
        let response = ajars
            .request(&PING)           // <-- Here's everything required
            .send(&PingRequest {})
            .await
            .unwrap();
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```

### Surf
To use it with [surf](https://github.com/http-rs/surf) enable the `surf` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["surf"] }
```

Example:
```rust
#[cfg(feature = "surf")]
mod surf { 
    use ajars::Rest;
    use ajars::surf::AjarsClientSurf;
    use serde::{Deserialize, Serialize};

    pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");

    async fn client() {

        let ajars = AjarsClientSurf::new(ajars::surf::surf::client(), "http://127.0.0.1:8080");
        
        let response = ajars
            .request(&PING)            // <-- Here's everything required
            .send(&PingRequest { })
            .await
            .unwrap();
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```

## Supported servers

### Actix-web
To use with [actix-web](https://github.com/actix/actix-web) enable the `actix_web` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["actix_web"] }
```

Example:
```rust
#[cfg(feature = "actix_web")]
mod actix_web { 
    use ajars::Rest;
    use serde::{Deserialize, Serialize};
    use ajars::actix_web::AjarsServerActixWebHandler;
    use ajars::actix_web::actix_web::{App, HttpServer, ResponseError};
    use derive_more::{Display, Error};

    pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");

    async fn server() {

        HttpServer::new(move || 
            App::new().service(
                PING.to(ping)    // <-- Here's everything required
            )
        )
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap();

    }

    #[derive(Debug, Display, Error)]
    enum UserError {
        #[display(fmt = "Validation error on field: {}", field)]
        ValidationError { field: String },
    }
    impl ResponseError for UserError {}

    async fn ping(_body: PingRequest) -> Result<PingResponse, UserError> {
        Ok(PingResponse {})
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```

### Axum
To use with [axum](https://github.com/tokio-rs/axum) enable the `axum` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["axum"] }
```

Example:
```rust
#[cfg(feature = "axum")]
mod axum { 
    use ajars::Rest;
    use ajars::axum::axum::{body::{Body, HttpBody}, http::Response, response::IntoResponse, Router};
    use ajars::axum::AjarsServerAxumHandler;
    use serde::{Deserialize, Serialize};
    use std::net::SocketAddr;
    use derive_more::{Display, Error};
    use tokio::net::TcpListener;

    pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");

    async fn server() {

        let app = Router::new()
                .merge(PING.to(ping));  // <-- Here's everything required

            let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

            println!("Start axum to {}", addr);

            let listener = TcpListener::bind(&addr).await.unwrap();
            ajars::axum::axum::serve(listener, app.into_make_service()).await.unwrap();

    }

    #[derive(Debug, Display, Error)]
    enum UserError {
        #[display(fmt = "Validation error on field: {}", field)]
        ValidationError { field: String },
    }

    impl IntoResponse for UserError {
        fn into_response(self) -> Response<Body> {
            Response::new(Body::empty())
        }
    }

    async fn ping(_body: PingRequest) -> Result<PingResponse, UserError> {
        Ok(PingResponse {})
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingRequest {}

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PingResponse {}
}
```