/*
use std::{future::Future, marker::PhantomData};

use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};
use ::warp::Filter;

pub mod warp {
    pub use warp::*;
}


pub fn handle<REST, I, O>(rest: &REST) ->  impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
where REST: RestType<I, O>,
      I: Serialize + DeserializeOwned + 'static, 
      O: Serialize + DeserializeOwned + 'static,
{ 

    let mut filter = ::warp::path(rest.path());

    // This does not compile
    match rest.method() {
        HttpMethod::DELETE => filter.and(warp::delete()),
        HttpMethod::GET => filter.and(warp::get()),
        HttpMethod::POST => filter.and(warp::post()),
        HttpMethod::PUT => filter.and(warp::put()),
    }
}

*/