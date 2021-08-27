#![doc = include_str!("../README.md")]

#[cfg(feature = "actix_web")]
pub mod actix_web {
    pub use ajars_actix_web::*;
}

#[cfg(feature = "axum")]
pub mod axum {
    pub use ajars_axum::*;
}

#[cfg(feature = "reqwest")]
pub mod reqwest {
    pub use ajars_reqwest::*;
}

#[cfg(feature = "surf")]
pub mod surf {
    pub use ajars_surf::*;
}

#[cfg(feature = "web")]
pub mod web {
    pub use ajars_web::*;
}

pub use ajars_core::*;
