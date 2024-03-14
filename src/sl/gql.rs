use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use graphql_client::Response;
use wasm_bindgen::JsValue;

/// Represents an error encountered during a fetch operation in a WebAssembly environment.
///
/// This struct encapsulates the JavaScript error (`JsValue`) that occurred during the fetching process,
/// making it easier to handle fetch errors within Rust code in a WebAssembly project. `FetchError`
/// implements the `std::fmt::Display` and `std::error::Error` traits, allowing it to integrate seamlessly
/// with Rust's error handling mechanisms.
///
/// # Fields
///
/// - `err`: The underlying JavaScript error (`JsValue`) that caused the fetch operation to fail.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    pub err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<String> for FetchError {
    fn from(value: String) -> Self {
        let js_value_error = JsValue::from_str(&value);
        FetchError { err: js_value_error }
    }
}
impl From<serde_json::Error> for FetchError {
    fn from(value: serde_json::Error) -> Self {
        // Convert the serde_json::Error to a string and then to a JsValue
        let error_message = value.to_string();
        let js_value_error = JsValue::from_str(&error_message);

        FetchError { err: js_value_error }
    }
}


impl From<reqwest::Error> for FetchError {
    fn from(value: reqwest::Error) -> Self {
        // Convert the reqwest::Error to a string and then to a JsValue
        let error_message = value.to_string();
        let js_value_error = JsValue::from_str(&error_message);

        FetchError { err: js_value_error }
    }
}


/// Fetches a text response from a GQL JSON request in a WebAssembly environment.
///
/// This asynchronous function sends an HTTP request and waits for its text response. It is designed
/// to work within the Yew framework and utilizes the `web_sys` and `js_sys` crates for Web API interactions.
///
/// # Arguments
///
/// * `gql_query_body`: GQL JSON string to be sent via an HTTP.
///
/// # Returns
///
/// * `Result<String, FetchError>`: On success, returns the response body as a `String`. On failure, returns
/// a `FetchError` indicating what went wrong during the request process.
pub async fn post_gql_query(gql_query_body: String) -> Result<String, FetchError> {

    /// GQL endpoint expected on the BE server.
    pub static GQL_URL: &str = "http://127.0.0.1:3001/gql";

    let client = reqwest::Client::new();
    let mut res = client.post(GQL_URL).body(gql_query_body).send().await?;

    Ok(res.text().await?)
}