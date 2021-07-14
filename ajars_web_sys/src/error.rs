use thiserror::Error as ThisError;
use wasm_bindgen::JsValue;


#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Cannot find Window object")]
    MissingWindow,
  #[error("an HTTP error occurred")]
  Http(
    #[from]
    #[source]
    http::Error,
  ),
  #[error("encountered request header with opaque bytes")]
  HttpHeader(
    #[from]
    #[source]
    http::header::ToStrError,
  ),
  #[error("an invalid HTTP status was received")]
  InvalidStatusCode(
    #[from]
    #[source]
    http::status::InvalidStatusCode,
  ),
  #[error("{context}")]
  WebSys {
    /// Some crate-provided context to the error.
    context: String,
    /// The originally reported `JsValue` in some textual form.
    // We do not keep the `JsValue` around because they are a pain to
    // work with (just extracting something useful) and they cause
    // everything they touch to be not `Send`.
    #[source]
    source: WebError,
  },
}

#[derive(Debug, ThisError)]
#[error("{0}")]
pub struct WebError(String);


impl Error {
  /// Create a new `Error::WebSys` variant.
  pub(crate) fn web<S>(context: S, error: JsValue) -> Error
  where
    S: Into<String>,
  {
    let error = if let Some(error) = error.as_string() {
      error
    } else {
      format!("{:?}", error)
    };

    Self::WebSys {
      context: context.into(),
      source: WebError(error),
    }
  }
}