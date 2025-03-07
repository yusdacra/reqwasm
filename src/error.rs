use gloo_utils::errors::JsError;
use thiserror::Error as ThisError;

/// All the errors returned by this crate.
#[derive(Debug, ThisError)]
pub enum Error {
    /// Error returned by JavaScript.
    #[error("{0}")]
    JsError(JsError),
    /// Error returned by `serde` during deserialization.
    #[cfg(feature = "json")]
    #[error("{0}")]
    SerdeError(
        #[source]
        #[from]
        serde_json::Error,
    ),
}

#[cfg(any(feature = "http", feature = "websocket"))]
pub(crate) use conversion::*;
#[cfg(any(feature = "http", feature = "websocket"))]
mod conversion {
    use gloo_utils::errors::JsError;
    use wasm_bindgen::JsValue;

    #[cfg(feature = "http")]
    pub(crate) fn js_to_error(js_value: JsValue) -> super::Error {
        super::Error::JsError(js_to_js_error(js_value))
    }

    pub(crate) fn js_to_js_error(js_value: JsValue) -> JsError {
        match JsError::try_from(js_value) {
            Ok(error) => error,
            Err(_) => unreachable!("JsValue passed is not an Error type -- this is a bug"),
        }
    }
}
