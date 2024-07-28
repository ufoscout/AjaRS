use thiserror::Error as ThisError;

use crate::HttpStatus;

#[derive(Debug, ThisError)]
pub enum Error {

    #[error("Builder error. Context: {context}. Error: {error}")]
    Builder {
        /// Some crate-provided context to the error.
        context: String,
        /// The originally reported error.
        error: String,
    },
    #[error("Response error. HTTP status: {status}. Context: {context}. Error: {error}")]
    Response {
        status: HttpStatus,
        /// Some crate-provided context to the error.
        context: String,
        /// The originally reported error.
        error: String,
    },
}
