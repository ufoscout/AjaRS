use std::{future::Future, marker::PhantomData};

use actix_web::{*, dev::Handler as ActixHandler, web::{Data, Json}};
use serde::{Serialize, de::DeserializeOwned};

use crate::Rest;

impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static> Rest<I, O> {

    pub fn handle<H, D, R, E>(&self, handler: H) -> Resource
    where
        H: Handler<D, R, E, I, O>,
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

pub trait Handler<D, R, E, I, O>: Clone + 'static
where
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static, 
    O: Serialize + DeserializeOwned + 'static
{
    fn call(&self, param: (HttpRequest, Data<D>, Json<I>)) -> R;
}

impl <F, D, R, E, I, O> Handler<D, R, E, I, O> for F
where
    F: 'static + Clone + Fn(HttpRequest, Data<D>, I) -> R,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static, 
    O: Serialize + DeserializeOwned + 'static
{
    fn call(&self, param: (HttpRequest, Data<D>, Json<I>)) -> R {
        self(param.0, param.1, param.2.into_inner())
    }
}

pub struct HandlerWrapper<H, D, R, E, I, O> 
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static, 
    O: Serialize + DeserializeOwned + 'static {

        handler: H,
        phantom_d: PhantomData<D>,
        phantom_r: PhantomData<R>,
        phantom_e: PhantomData<E>,
        phantom_i: PhantomData<I>,
        phantom_o: PhantomData<O>,
}

impl <H, D, R, E, I, O> HandlerWrapper<H, D, R, E, I, O> 
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static, 
    O: Serialize + DeserializeOwned + 'static {

    pub fn new(handler: H) -> Self {
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

impl <H, D, R, E, I, O> Clone for HandlerWrapper<H, D, R, E, I, O> 
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static, 
    O: Serialize + DeserializeOwned + 'static {

    fn clone(&self) -> Self {
        Self::new(self.handler.clone())
    }
}


impl <H, D, R, E, I, O> ActixHandler<(HttpRequest, Data<D>, Json<I>), R> for HandlerWrapper<H, D, R, E, I, O> 
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    D: 'static,
    R: Future<Output = Result<Json<O>, E>> + 'static,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static, 
    O: Serialize + DeserializeOwned + 'static, {

    fn call(&self, param: (HttpRequest, Data<D>, Json<I>)) -> R {
        self.handler.call(param)
    }
}

