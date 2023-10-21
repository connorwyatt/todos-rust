pub(crate) mod latency;
pub(crate) mod middleware;

use std::{net::SocketAddr, time::Duration};

use axum::{Router, Server};
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
    ServiceBuilderExt,
};

use crate::server::middleware::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};

pub(crate) async fn start(router: Router) {
    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default())
                .on_request(DefaultOnRequest::default())
                .on_response(DefaultOnResponse::default()),
        )
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .propagate_x_request_id()
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .compression();

    let app = router.layer(middleware);

    let server =
        Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000))).serve(app.into_make_service());

    tracing::debug!("listening on {}", server.local_addr());

    server.await.unwrap();
}
