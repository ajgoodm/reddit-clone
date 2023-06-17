use axum::{
    routing::{get, post},
    Router,
};

use reddit_clone::{
    database::get_connection_pool,
    get_serving_address,
    handlers::{create_user, get_user},
};

#[tokio::main]
async fn main() {
    let connection_pool = get_connection_pool().await;
    let app = Router::new()
        .route("/users", post(create_user))
        .route("/users/:user_id", get(get_user))
        .with_state(connection_pool);

    let address = get_serving_address();
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
