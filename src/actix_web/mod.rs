use std::{future::Future, marker::PhantomData};

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

        web::resource::<&str>(self.path.as_ref()).route(route.to(HandlerWrapper::new(handler)))
    }
    
}

pub struct HandlerWrapper<T, R, H> 
where
    H: Handler<T, R> + Clone + 'static,
    R: Future,
    R::Output: Responder, {

        handler: H,
        phantom_r: PhantomData<R>,
        phantom_t: PhantomData<T>
}

impl <T, R, H> HandlerWrapper<T, R, H> 
where
    H: Handler<T, R> + Clone + 'static,
    R: Future,
    R::Output: Responder, {

    pub fn new(handler: H) -> Self {
        Self {
            handler,
            phantom_r: PhantomData,
            phantom_t: PhantomData,
        }
    }
}

impl <T, R, H> Clone for HandlerWrapper<T, R, H>
where
    H: Handler<T, R> + Clone + 'static,
    R: Future,
    R::Output: Responder, {
    fn clone(&self) -> Self {
        Self::new(self.handler.clone())
    }
}

impl <T: 'static, R: 'static, H: 'static> Handler<T, R> for HandlerWrapper<T, R, H> 
where
    H: Handler<T, R> + Clone + 'static,
    R: Future,
    R::Output: Responder,
{
    fn call(&self, param: T) -> R {
        self.handler.call(param)
    }
}

