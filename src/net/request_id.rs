use axum::http::{HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};
use tracing::{instrument, trace};
use ulid::Ulid;

#[derive(Clone, Copy, Default)]
pub struct MakeRequestUlid;

impl MakeRequestId for MakeRequestUlid {
    #[instrument(skip(self, request), level = "trace")]
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_ulid = Ulid::new().to_string();
        trace!("Generated request ID: {}", request_ulid);

        let header_value: HeaderValue = request_ulid.parse().unwrap();
        Some(RequestId::new(header_value))
    }
}
