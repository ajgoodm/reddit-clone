pub mod database;
pub mod handlers;
pub mod models;
pub mod schema;
pub mod utils;

use std::net::SocketAddr;

pub fn get_serving_address() -> SocketAddr {
    let addr_details = std::env::var("RC_API_ADDR").expect("RC_SERVING_HOST must be set");

    addr_details
        .parse::<SocketAddr>()
        .expect("Invalid API serving address")
}
