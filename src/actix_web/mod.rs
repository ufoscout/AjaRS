use std::{future::Future, marker::PhantomData};

use actix_web::{*, dev::Handler, web::{Data, Json}};
use serde::{Serialize, de::DeserializeOwned};

use crate::Rest;

impl <I: DeserializeOwned + Clone + 'static, O: Serialize + Clone + 'static> Rest<I, O> {

    pub fn to_resource<H: Clone + 'static, D: Clone + 'static, R: Clone + 'static, E: Clone + 'static>(&self, handler: H) -> Resource 
    where 
        H: Fn(HttpRequest, Data<D>, Json<I>) -> R,
        R: Future<Output = Result<Json<O>, E>>,
        E: ResponseError + 'static
    {
        let route = match self.method {
            crate::HttpMethod::GET => web::get(),
            crate::HttpMethod::POST => web::post(),
        };

        web::resource::<&str>(self.path.as_ref()).route(route).to(MyHandler::new(handler))
    }

}

#[derive(Clone)]
struct MyHandler<H, D, R, E, I, O > 
where 
    H: Fn(HttpRequest, Data<D>, Json<I>) -> R,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static {
        pub handler: H,
        phantom_d: PhantomData<D>,
        phantom_r: PhantomData<R>,
        phantom_e: PhantomData<E>,
        phantom_i: PhantomData<I>,
        phantom_o: PhantomData<O>,
}

impl <H, D, R, E, I, O > MyHandler<H, D, R, E, I, O >
where 
    H: Fn(HttpRequest, Data<D>, Json<I>) -> R,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static {

        fn new(handler: H) -> Self {
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

impl <H: Clone + 'static, D: Clone + 'static, R: Clone + 'static, E: Clone + 'static, I: DeserializeOwned + 'static + Clone, O: Serialize + Clone + 'static> 
    Handler<(HttpRequest, Data<D>, Json<I>), R> for MyHandler<H, D, R, E, I, O >
where 
    H: Fn(HttpRequest, Data<D>, Json<I>) -> R,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static {

    fn call(&self, param: (HttpRequest, Data<D>, Json<I>)) -> R {
        (self.handler)(param.0, param.1, param.2)
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
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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