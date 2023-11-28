#![doc = include_str!("../README.md")]

#[cfg(feature = "actix_web")]
pub mod actix_web {
    pub use ajars_server_actix_web::*;
}

#[cfg(feature = "axum")]
pub mod axum {
    pub use ajars_server_axum::*;
}

#[cfg(feature = "reqwest")]
pub mod reqwest {
    pub use ajars_client_reqwest::*;
}

#[cfg(feature = "surf")]
pub mod surf {
    pub use ajars_client_surf::*;
}

#[cfg(feature = "web")]
pub mod web {
    pub use ajars_client_web::*;
}

pub use ajars_core::*;
