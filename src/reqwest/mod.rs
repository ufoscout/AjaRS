use serde::{Serialize, de::DeserializeOwned};

use crate::Rest;

impl <I: DeserializeOwned, O: Serialize> Rest<I, O> {

    /*
    pub fn reqwest(&self) -> Resource {
        let route = match self.method {
            crate::HttpMethod::GET => web::get(),
            crate::HttpMethod::POST => web::post(),
        };
        web::resource::<&str>(self.path.as_ref()).route(route)
    }
    */

}