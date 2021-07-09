use std::future::Future;

use actix_web::{*, dev::Handler, web::{Data, Json}};
use serde::{Serialize, de::DeserializeOwned};

use crate::Rest;

impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static> Rest<I, O> {

    pub fn handle<F, D, R, E>(&self, handler: F) -> Resource
    where
        F: Handler<(HttpRequest, Data<D>, Json<I>), R>,
        D: 'static,
        R: Future<Output = Result<Json<O>, E>> + 'static,
        E: ResponseError + 'static
    {
        let route = match self.method {
            crate::HttpMethod::GET => {
                web::get()
            },
            crate::HttpMethod::POST => {
                web::post()
            },
        };

        web::resource::<&str>(self.path.as_ref()).route(route.to(handler))
    }
    
}

#[cfg(test)]
mod test {
    
    use crate::reqwest::RestReqwest;
    use actix_web::{App, HttpServer};
    use actix_rt::spawn;
    use actix_rt::time::sleep;
    use serde::{Deserialize};
    use std::fmt::Display;
    use std::time::Duration;
    use super::*;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
    struct Simple<O> {
        pub inner: O
    }

    #[derive(Debug, Clone)]
    struct MyError {}

    impl Display for MyError {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    
    impl error::ResponseError for MyError {}

    #[cfg(feature = "client_reqwest")]
    #[actix_rt::test]
    async fn const_builder() {

        let rest = Rest::<Simple<String>, Simple<String>>::post("/api/hello/world");
        
        let free_port = port_check::free_local_port().unwrap();
        let address = format!("127.0.0.1:{}", free_port);

        // Start Server
        let address_clone = address.clone();
        let rest_clone = rest.clone();
         
        spawn(async move {
            println!("Start actix-web to {}", address_clone);
            HttpServer::new(move || {
                App::new()
                    .app_data(Data::new(()))
                    .service(rest_clone.handle(post_hello_world))
            })
            .bind(&address_clone).and_then(|ser| {
                Ok(ser)
            }).unwrap().run().await.unwrap();
        });

        sleep(Duration::from_millis(200)).await;

        // Start client
        let req = RestReqwest::new(reqwest::ClientBuilder::new().build().unwrap(), format!("http://{}", address));

        let req_data = Simple {
            inner: format!("{}", rand::random::<usize>())
        };
        let response = req.submit(&rest, &req_data).await;
        println!("Response: {:?}", response);
        assert_eq!(req_data, response.unwrap());
    }

    async fn post_hello_world(_request: HttpRequest, _data: Data<()>, body: Json<Simple<String>>) -> Result<Json<Simple<String>>, MyError> {
        println!("Request body: {:?}", body);
        Ok(body)
    }

}