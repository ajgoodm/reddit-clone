use axum::{routing::get, Router};

use reddit_clone::{
    database::get_connection_pool,
    get_serving_address,
    handlers::{create_user, list_users},
};

#[tokio::main]
async fn main() {
    let connection_pool = get_connection_pool().await;
    let app = Router::new()
        .route("/users", get(list_users).post(create_user))
        .with_state(connection_pool);

    let address = get_serving_address();
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
