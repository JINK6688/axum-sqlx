use axum::Router;
use tower::ServiceBuilder;

use crate::jwt::JwtLayer;
pub mod ctx;
pub mod jwt;

/// Simple request-id + trace layer using tower-http's request_id feature
pub fn apply(router: Router) -> Router {
    use tower_http::{
        cors::CorsLayer,
        request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
        trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    };

    // Trace HTTP traffic
    let trace = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().include_headers(true))
        .on_response(DefaultOnResponse::new());

    // Set & propagate request IDs
    let req_id = SetRequestIdLayer::x_request_id(MakeRequestUuid);
    let propagate = PropagateRequestIdLayer::x_request_id();

    // Set cors headers
    let core = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    //build the middleware stack
    let layer = ServiceBuilder::new()
        .layer(trace)
        .layer(req_id)
        .layer(propagate)
        .layer(core)
        .layer(JwtLayer::new());

    router.layer(layer)
}
