pub(crate) mod extensions;
pub(crate) mod logging;
pub(crate) mod todos;

use axum::Router;

use crate::todos::api::routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file is missing");

    logging::initialize_logging();

    let app = Router::new().merge(routes::router());

    server::start(server::create_router(extensions::add(app).await), 3000).await;
}
