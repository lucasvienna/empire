use axum::http::Request;
use tower_http::request_id::{MakeRequestId, RequestId};
use ulid::Ulid;

#[derive(Clone, Copy, Default)]
pub struct MakeRequestUlid;

impl MakeRequestId for MakeRequestUlid {
    fn make_request_id<B>(&mut self, request: &Request<B>) -> Option<RequestId> {
        let request_ulid = Ulid::new().to_string().parse().unwrap();
        Some(RequestId::new(request_ulid))
    }
}