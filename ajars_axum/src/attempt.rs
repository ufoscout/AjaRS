use std::future::Future;

use serde::{Serialize, de::DeserializeOwned};


trait One {}
impl One for &str {}

trait Two<O> {
    fn do_something(&self);
}

impl <F> Two<()> for F where F: Fn() + Clone {
    fn do_something(&self) {
        todo!()
    }
}

impl <F, O: One> Two<(O)> for F where F: Fn(O) {
    fn do_something(&self) {
        todo!()
    }
}


impl <F, O1: One, O2: One> Two<(O1, O2)> for F where F: Fn(O1, O2) {
    fn do_something(&self) {
        todo!()
    }
}



pub trait IntoResponse{}

pub trait FromRequest<T>{}

pub trait Handler<B, T> {
    fn route(self);
}

impl <B, F, R, E, P1> Handler<B, (P1,)> for F
where 
    F: 'static + Send + Sync + FnOnce(P1) -> R,
    R: Future<Output = Result<E, E>> + Send,
    E: IntoResponse + Send + 'static,
    B: Send + 'static,
    P1: FromRequest<B>
{
    fn route(self) {
        todo!()
    }
}

impl <B, F, R, E, P1, P2> Handler<B, (P1, P2)> for F
where 
    F: 'static + Send + Sync + FnOnce(P1) -> R,
    R: Future<Output = Result<E, E>> + Send,
    E: IntoResponse + Send + 'static,
    B: Send + 'static,
    P1: FromRequest<B> + Send,
    P2: FromRequest<B> + Send,
{
    fn route(self) {
        todo!()
    }
}