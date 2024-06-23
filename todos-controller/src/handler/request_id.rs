use http::{HeaderName, HeaderValue, Request};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

pub const ONE_KIT_REQUEST_ID: HeaderName = HeaderName::from_static("one-kit-request-id");

#[derive(Clone)]
pub struct OneKitRequestId {}

impl OneKitRequestId {
    pub fn new() -> Self {
        OneKitRequestId {}
    }
}

impl MakeRequestId for OneKitRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        // generate uuid at every request
        let uuid = Uuid::new_v4().to_string();
        let request_id = HeaderValue::from_str(&uuid).unwrap();

        //tracing::debug!("{:#?}", request_id);

        Some(RequestId::new(request_id))
    }
}
