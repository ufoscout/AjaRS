use ajars::surf::{surf, AjarsSurf};
use examples_common::{
    hello::{HelloRequest, HELLO},
    ping::{PingRequest, PING},
};

#[tokio::main]
async fn main() {
    let ajars = AjarsSurf::new(surf::client(), "http://127.0.0.1:8080".to_owned());

    // PING
    {
        // Performs a GET request to http://127.0.0.1:8080/api/ping
        // The PingRequest and PingResponse types are enforced at compile time
        let response = ajars
            .request(&PING)
            .send(&PingRequest { message: "Surf".to_owned() })
            .await
            .expect("Should perform a GET call. Is the server running?");

        println!("\nPing call performed.\nResponse: {:?}\n", response);
    }

    // HELLO
    {
        // Performs a POST request to http://127.0.0.1:8080/api/hello
        // The HelloRequest and HelloResponse types are enforced at compile time
        let response = ajars
            .request(&HELLO)
            .send(&HelloRequest { names: vec!["Francesco".to_owned(), "Luke".to_owned(), "Mary".to_owned()] })
            .await
            .expect("Should perform a POST call. Is the server running?");

        println!("\nHello call performed.\nResponse: {:?}\n", response);
    }
}
