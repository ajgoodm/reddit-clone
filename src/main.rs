use axum::{routing::get, Router};
use std::net::SocketAddr;

use reddit_clone::{database::get_connection_pool, handlers::list_users};

#[tokio::main]
async fn main() {
    let connection_pool = get_connection_pool().await;
    let app = Router::new()
        .route("/users", get(list_users))
        .with_state(connection_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
