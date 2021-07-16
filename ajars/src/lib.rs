#[cfg(feature = "actix_web")]
pub mod actix_web {
    pub use ajars_actix_web::*;
}

#[cfg(feature = "reqwest")]
pub mod reqwest {
    pub use ajars_reqwest::*;
}

#[cfg(feature = "surf")]
pub mod surf {
    pub use ajars_surf::*;
}

#[cfg(feature = "web_sys")]
pub mod web_sys {
    pub use ajars_web_sys::*;
}

pub use ajars_core::*;
