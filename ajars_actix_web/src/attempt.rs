use std::future::Future;

use actix_web::{FromRequest, Resource, ResponseError, web::{self, Json}};
use ajars_core::{HttpMethod, RestType};
use serde::{Serialize, de::DeserializeOwned};

pub trait ActixWebHandler<I: Serialize + DeserializeOwned, O: Serialize + DeserializeOwned, T, H> {
    fn handle(self, handler: H) -> Resource;
}


impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, H, R, E, REST: RestType<I, O>> ActixWebHandler<I, O, (), H>
    for REST 
where 
H: Clone + 'static + Fn(I) -> R,
R: Future<Output = Result<Json<O>, E>> + 'static,
E: ResponseError + 'static,
{
    fn handle(self, handler: H) -> Resource {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => resource.route(web::delete().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
            HttpMethod::GET => resource.route(web::get().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
            HttpMethod::POST => resource.route(web::post().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
            HttpMethod::PUT => resource.route(web::put().to(
                move |json: Json<I>| {
                (handler)(json.into_inner())
            })),
        }
    }
}


impl <I: Serialize + DeserializeOwned + 'static, O: Serialize + DeserializeOwned + 'static, H, R, E, P0, REST: RestType<I, O>> ActixWebHandler<I, O, (P0,), H>
    for REST 
where 
H: Clone + 'static + Fn(I, P0) -> R,
R: Future<Output = Result<Json<O>, E>> + 'static,
E: ResponseError + 'static,
P0: FromRequest + 'static
{
    fn handle(self, handler: H) -> Resource {
        let resource = web::resource::<&str>(self.path());

        match self.method() {
            HttpMethod::DELETE => resource.route(web::delete().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
            HttpMethod::GET => resource.route(web::get().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
            HttpMethod::POST => resource.route(web::post().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
            HttpMethod::PUT => resource.route(web::put().to(
                move |json: Json<I>, p0: P0| {
                (handler)(json.into_inner(), p0)
            })),
        }
    }
}