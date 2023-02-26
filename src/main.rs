use axum::Router;
use std::net::SocketAddr;
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() {
    let port_arg = std::env::args()
        .position(|arg| arg.as_str() == "--port")
        .expect("--port <Number> needs to be provided")
        + 1;
    let port = std::env::args()
        .nth(port_arg)
        .expect("Port number needs to be provided --port <Number>")
        .parse::<u16>()
        .expect("Port number needs to be a number");

    let dir_arg = std::env::args()
        .position(|arg| arg.as_str() == "--dir")
        .expect("--dir <Path> needs to be provided")
        + 1;
    let dir = std::env::args()
        .nth(dir_arg)
        .expect("Dir path needs to be provided --port <Path>");

    let filter = std::env::args()
        .position(|arg| arg.as_str() == "--filter")
        .map(|i| {
            let filter_arg = i + 1;
            std::env::args()
                .nth(filter_arg)
                .expect("Filter needs to be provided --filter <Filter>")
        })
        .unwrap_or("sfs=debug,tower_http=debug".into());

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = Router::new().nest_service("/", ServeDir::new(&dir));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.layer(TraceLayer::new_for_http()).into_make_service())
        .await
        .unwrap();
}
