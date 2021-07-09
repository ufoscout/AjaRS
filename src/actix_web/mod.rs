use std::future::Future;

use actix_web::{*, dev::Handler, web::{Data, Json}};
use serde::{Serialize, de::DeserializeOwned};

use crate::Rest;

impl <I: DeserializeOwned + Clone + 'static, O: Serialize + Clone + 'static> Rest<I, O> {

    pub fn to_resource<F, D, R>(&self, handler: F) -> Resource
    where
        F: Handler<(HttpRequest, Data<D>, Json<I>), R>,
        D: 'static,
        R: Future + 'static,
        R::Output: Responder + 'static,
    {
        let route = match self.method {
            crate::HttpMethod::GET => web::get(),
            crate::HttpMethod::POST => web::post(),
        };

        web::resource::<&str>(self.path.as_ref()).route(route).to(handler)
    }

}

#[cfg(test)]
mod test {

    use std::fmt::Display;

    use actix_web::{App, HttpServer};

    use super::*;

    #[derive(Debug, Clone)]
    struct MyError {}

    impl Display for MyError {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    
    impl error::ResponseError for MyError {}

    #[test]
    fn const_builder() {
        let rest = Rest::<String, String>::get("/api/hello/world");
        
        let free_port = port_check::free_local_port().unwrap();
        let address = format!("0.0.0.0:{}", free_port);

        HttpServer::new(move || {
            App::new()
                .service(rest.to_resource(post_hello_world))
        })
        .bind(&address).unwrap();
    }

    async fn post_hello_world(request: HttpRequest, data: Data<String>, body: Json<String>) -> Result<Json<String>, MyError> {
        unimplemented!()
    }

}