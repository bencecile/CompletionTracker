pub mod data;
mod universe;

pub use self::universe::{UniverseEndpoint};

use serde::{Serialize};
use serde_json::{Value, json};

use rouille::{Response};

use crate::api::{RequestPipeline};
use crate::lang::{Translatable, UIStrings};

/// Sends the response to a shutdown.
/// Doesn't actually do anything for the shutdown.
pub fn shutdown_response() -> Response {
    log::info!(target: "Shutdown", "Sending the shutdown response");
    Response::json(
        &make_endpoint_response_json_success(UIStrings::new().shutdown_response)
    )
}

/// This is an endpoint for our API (sources, tracking, etc.).
pub trait ApiEndpoint {
    // The name of the endpoint
    const NAME: &'static str;

    /// Tries to match the request with this endpoint
    fn match_request<'a>(pipeline: &RequestPipeline<'a>) -> Option<Response>;


    /// Converts the given result from an endpoint and turns it into a full response.
    /// Logs the endpoint response from the result given by the closure.
    fn convert_endpoint_response<A, T, E>(endpoint_result: Result<T, E>) -> Response
    where T: Serialize, E: Translatable, A: EndpointTarget {
        // Logs the result and gets at the value inside
        match endpoint_result {
            Ok(success_data) => Self::convert_endpoint_success::<A, T>(success_data),
            Err(err) => Self::convert_endpoint_failure::<A, E>(err),
        }
    }

    /// Create a response from the successful data
    fn convert_endpoint_success<A, T>(success_data: T) -> Response
    where T: Serialize, A: EndpointTarget {
        // Make the json from the data
        let json_value = make_endpoint_response_json_success(success_data);
        // Log the success
        log::info!(target: Self::NAME, "{} Success: {:?}", A::ENDPOINT_TARGET, &json_value);
        Response::json(&json_value)
    }
    fn convert_endpoint_failure<A, E>(err: E) -> Response
    where E: Translatable, A: EndpointTarget {
        let json_value = make_endpoint_response_json_failure(err);
        log::warn!(target: Self::NAME, "{} Failure: {:?}", A::ENDPOINT_TARGET, &json_value);
        Response::json(&json_value)
            .with_status_code(500)
    }
}
trait EndpointTarget {
    /// The target of this endpoint
    const ENDPOINT_TARGET: &'static str;
}

// ------ The different possible endpoint targets ------
struct Create;
impl EndpointTarget for Create { const ENDPOINT_TARGET: &'static str = "CREATE"; }
struct Get;
impl EndpointTarget for Get { const ENDPOINT_TARGET: &'static str = "GET"; }


/// The JSON that is always the body of a successful Response
fn make_endpoint_response_json_success<T>(data: T) -> Value
where T: Serialize {
    json!({
        "success": true,
        "data": data,
    })
}
/// The JSON that is always the body of a failed Response
fn make_endpoint_response_json_failure<E>(err: E) -> Value
where E: Translatable {
    json!({
        "success": false,
        "errorMessage": err.to_lang_strings(),
    })
}
