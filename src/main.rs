pub(crate) mod middleware;
pub(crate) mod todos;

use std::{net::SocketAddr, time::Duration};

use axum::Router;
use tower::ServiceBuilder;
use tower_http::{
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::TraceLayer,
    validate_request::ValidateRequestHeaderLayer,
    ServiceBuilderExt,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::middleware::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todos=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
        .map_response_body(axum::body::boxed)
        .layer(ValidateRequestHeaderLayer::accept("application/json"))
        .compression();

    let app = Router::new()
        .merge(todos::api::routes::router())
        .layer(middleware);

    let server = axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 3000)))
        .serve(app.into_make_service());
    tracing::debug!("listening on {}", server.local_addr());
    server.await.unwrap();
}
