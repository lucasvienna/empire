/// Creates an HTTP 401 Unauthorized response with a JSON body.
///
/// # Arguments
/// * `$body` - The response body to be serialized as JSON
/// * `$jar` - (Optional) A cookie jar to be included in the response
///
/// # Returns
/// A response with status code 401 and the provided JSON body
#[macro_export]
macro_rules! unauthorized {
    ($body:expr) => {
        (StatusCode::UNAUTHORIZED, Json($body)).into_response()
    };
    ($body:expr, $jar:expr) => {
        (
            $jar,
            (StatusCode::UNAUTHORIZED, Json($body)).into_response(),
        )
    };
}

/// Creates an HTTP 200 OK response with a JSON body.
///
/// # Arguments
/// * `$body` - The response body to be serialized as JSON
/// * `$jar` - (Optional) A cookie jar to be included in the response
///
/// # Returns
/// A response with status code 200 and the provided JSON body
#[macro_export]
macro_rules! okay {
    ($body:expr) => {
        (StatusCode::OK, Json($body)).into_response()
    };
    ($body:expr, $jar:expr) => {
        ($jar, (StatusCode::OK, Json($body)).into_response())
    };
}
