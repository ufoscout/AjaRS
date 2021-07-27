use std::{future::Future, marker::PhantomData};

use ajars_core::{HttpMethod, RestType};
use serde::{de::DeserializeOwned, Serialize};

pub mod warp {
    pub use warp::*;
}

