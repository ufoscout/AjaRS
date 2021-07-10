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
