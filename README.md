# AjaRS

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
use ajars::actix_web::actix_web::{App, Result, web::{self, Json}};
use serde::{Deserialize, Serialize};

App::new().service(

    web::resource("/ping")  // PATH definition here
    
    .route(web::post()     // HTTP Method definition here
    
    .to(ping)              // The signature of the `ping` fn determines the
                        // JSON types produced and consumed. In this case
                        // PingRequest and PingResponse
    )
);

async fn ping(_body: Json<PingRequest>) -> Result<Json<PingResponse>> {
    Ok(Json(PingResponse { message: "PONG".to_owned() }))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub message: String,
}
```

Let's now declare a client using [reqwest](https://github.com/seanmonstar/reqwest)
```rust
use ajars::reqwest::reqwest::ClientBuilder;
use serde::{Deserialize, Serialize};

pub async fn client() {
    let client = ClientBuilder::new().build().unwrap();

    let url = "http://127.0.0.1:8080/ping";            // Duplicated '/ping' path definition
    
    client.post(url)                                   // Duplicated HTTP Post method definition
    
    .json(&PingRequest { message: "PING".to_owned() }) // Duplicated request type. Not checked at compile time
    
    .send().await.unwrap()
    .json::<PingResponse>().await.unwrap();            // Duplicated response type. Not checked at compile time
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingRequest {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PingResponse {
    pub message: String,
}
```

Wouldn't it be good to have those values declared only once with all types checked at compile time?

## The AjaRs solution

Ajars allows a single definition for those values. This removes code duplication and
allows compile time verification that the endpoint server and client path, method, request type and response type are coherent.

For example, the following is the Ajars definition of the previous endpoint, ideally declared in a commond library imported by both the server and the client:
```rust
use ajars::Rest;

// This defines a 'POST' call with path /ping, request type 'PingRequest' and response type 'PingResponse'
pub const PING: Rest<PingRequest, PingResponse> = Rest::post("/ping");
```

Now, using Ajars, the server side endpoint creation with actix-web becomes:
```rust
use ajars::actix_web::HandleActix;

HttpServer::new(move || 
        App::new().service(
            PING.handle(ping)
        )
    )
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
});
```

and the reqwest client becomes:
```rust
let ajars = AjarsReqwest::new(ClientBuilder::new().build().unwrap(), "http://127.0.0.1:8080");

// Performs a POST request to http://127.0.0.1:8080/ping
// The PingRequest and PingResponse types are enforced at compile time
let response = ajars
    .request(&PING)
    .send(&PingRequest { message: "Reqwest".to_owned() })
    .await
    .unwrap();
```

## Supported clients

### WASM in the browser
Ajars provides a lightweight client implementation based on [web-sys](TODO), 
this is to be used in WASM based web frontends that run in a browser (e.g. [Yew](TODO), [Sycamore](TODO), etc...).

To use it enable the `web` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["web"] }
```

Example:
```rust
use ajars::web::{error::Error, AjarsWeb};

let base_url = "";
let ajars = AjarsWeb::new(base_url).expect("Should build Ajars");

let response = ajars
    .request(&PING)
    .send(&PingRequest { message: "Reqwest".to_owned() })
    .await
    .unwrap();
```

### Reqwest
To use it enable the `reqwest` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["reqwest"] }
```

Example:
```rust
use ajars::reqwest::{reqwest::ClientBuilder, AjarsReqwest};

let base_url = "http://127.0.0.1:8080";
let ajars = AjarsReqwest::new(ClientBuilder::new().build().unwrap(), base_url);

let response = ajars
    .request(&PING)
    .send(&PingRequest { message: "Reqwest".to_owned() })
    .await
    .unwrap();
```

### Surf
To use it enable the `surf` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["surf"] }
```

Example:
```rust
use ajars::surf::{surf, AjarsSurf};

let base_url = "http://127.0.0.1:8080";
let ajars = AjarsSurf::new(surf::client(), base_url);

let response = ajars
    .request(&PING)
    .send(&PingRequest { message: "Reqwest".to_owned() })
    .await
    .unwrap();
```

## Supported servers

### Actix-web
To use it enable the `actix_web` feature, in the Cargo.toml file:
```toml
ajars = { version = "LAST_VERSION", features = ["actix_web"] }
```

Example:
```rust
HttpServer::new(move || 
        App::new().service(
            PING.handle(ping)
        )
    )
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap();
});
```

### Warp (Work In Progress)
Not yet ready