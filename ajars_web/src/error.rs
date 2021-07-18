use thiserror::Error as ThisError;
use wasm_bindgen::JsValue;

use crate::HttpStatus;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Cannot find Window object")]
    MissingWindow,
    #[error("Builder error. Context: {context}")]
    Builder {
        /// Some crate-provided context to the error.
        context: String,
        /// The originally reported `JsValue` in some textual form.
        #[source]
        source: WebError,
    },
    #[error("Response error. HTTP status: {status}. Context: {context}")]
    Response {
        status: HttpStatus,
        /// Some crate-provided context to the error.
        context: String,
        /// The originally reported `JsValue` in some textual form.
        #[source]
        source: WebError,
    },
}

#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct WebError(pub String);

impl From<JsValue> for WebError {
    fn from(value: JsValue) -> Self {
        let error = if let Some(error) = value.as_string() { error } else { format!("{:?}", value) };
        WebError(error)
    }
}

impl Error {
    /// Create a new `Error::WebSys` variant.
    pub(crate) fn response<S>(status: HttpStatus, context: S, error: JsValue) -> Error
    where
        S: Into<String>,
    {
        Self::Response { status, context: context.into(), source: error.into() }
    }
}
