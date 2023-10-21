pub(crate) mod extensions;
pub(crate) mod server;
pub(crate) mod todos;

use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::todos::api::routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file is missing");

    let file_appender = tracing_appender::rolling::hourly("logs", "logs");
    let (non_blocking_file_appender, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug,hyper=warn".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(non_blocking_file_appender),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .init();

    let app = Router::new().merge(routes::router());

    server::start(extensions::add(app).await).await;
}
