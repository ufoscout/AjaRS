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

    use actix_web::{App, HttpServer};

    use super::*;

    #[test]
    fn const_builder() {
        let rest = Rest::<String, String>::get("/api/hello/world");
        let free_port = port_check::free_local_port().unwrap();
        let address = format!("0.0.0.0:{}", free_port);

        HttpServer::new(move || {
            App::new()
                .service(web::auth::build_auth_api(Api::new(web_auth_service.clone(), oregold_module.oregold_auth_module.auth_api.clone())))
                .service(web::core::build_core_api(Api::new(web_auth_service.clone(), oregold_module.oregold_core_module.core_api.clone())))
                .service(web::logs::build_log_api(Api::new(web_auth_service.clone(), oregold_module.oregold_log_module.log_api.clone())))
                .service(web::market::build_market_api(web_auth_service.clone(), &oregold_module))
                .service(web::um::build_um_api(web_auth_service.clone(), &oregold_module))
        })
        .bind(&address).unwrap();
    }

}