use std::{future::Future, marker::PhantomData};

use crate::actix_web::{
    dev::Handler as ActixHandler,
    web::{Data, Json, Query},
    *,
};
use ajars_core::{HttpMethod, Rest};
use serde::{de::DeserializeOwned, Serialize};

pub mod actix_web {
    pub use actix_web::*;
}

pub trait HandleActix<I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static> {
    fn handle<H, D, R, E>(&self, handler: H) -> Resource
    where
        H: Handler<D, R, E, I, O>,
        D: 'static,
        R: Future<Output = Result<Json<O>, E>> + 'static,
        E: ResponseError + 'static;
}

impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, REST: Rest<I,O>> HandleActix<I, O>
    for REST
{
    fn handle<H, D, R, E>(&self, handler: H) -> Resource
    where
        H: Handler<D, R, E, I, O>,
        D: 'static,
        R: Future<Output = Result<Json<O>, E>> + 'static,
        E: ResponseError + 'static,
    {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => {
                resource.route(web::delete().to(QueryHandlerWrapper::new(handler)))
            }
            HttpMethod::GET => {
                resource.route(web::get().to(QueryHandlerWrapper::new(handler)))
            }
            HttpMethod::POST => {
                resource.route(web::post().to(JsonHandlerWrapper::new(handler)))
            }
            HttpMethod::PUT => {
                resource.route(web::put().to(JsonHandlerWrapper::new(handler)))
            }
        }
    }
}

pub trait Handler<D, R, E, I, O>: Clone + 'static
where
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, I)) -> R;
}

impl<F, D, R, E, I, O> Handler<D, R, E, I, O> for F
where
    F: 'static + Clone + Fn(HttpRequest, Data<D>, I) -> R,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, I)) -> R {
        self(param.0, param.1, param.2)
    }
}

pub struct JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    handler: H,
    phantom_d: PhantomData<D>,
    phantom_r: PhantomData<R>,
    phantom_e: PhantomData<E>,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<H, D, R, E, I, O> JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
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

impl<H, D, R, E, I, O> Clone for JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn clone(&self) -> Self {
        Self::new(self.handler.clone())
    }
}

impl<H, D, R, E, I, O> ActixHandler<(HttpRequest, Data<D>, Json<I>), R>
    for JsonHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    D: 'static,
    R: Future<Output = Result<Json<O>, E>> + 'static,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, Json<I>)) -> R {
        self.handler.call((param.0, param.1, param.2.into_inner()))
    }
}

pub struct QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    handler: H,
    phantom_d: PhantomData<D>,
    phantom_r: PhantomData<R>,
    phantom_e: PhantomData<E>,
    phantom_i: PhantomData<I>,
    phantom_o: PhantomData<O>,
}

impl<H, D, R, E, I, O> QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
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

impl<H, D, R, E, I, O> Clone for QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    R: Future<Output = Result<Json<O>, E>>,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn clone(&self) -> Self {
        Self::new(self.handler.clone())
    }
}

impl<H, D, R, E, I, O> ActixHandler<(HttpRequest, Data<D>, Query<I>), R>
    for QueryHandlerWrapper<H, D, R, E, I, O>
where
    H: Handler<D, R, E, I, O> + Clone + 'static,
    D: 'static,
    R: Future<Output = Result<Json<O>, E>> + 'static,
    E: ResponseError + 'static,
    I: Serialize + DeserializeOwned + 'static,
    O: Serialize + DeserializeOwned + 'static,
{
    fn call(&self, param: (HttpRequest, Data<D>, Query<I>)) -> R {
        self.handler.call((param.0, param.1, param.2.into_inner()))
    }
}
