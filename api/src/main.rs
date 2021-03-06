pub mod filters;
pub mod handlers;
pub mod models;

use redis::Client;
use std::env;
use warp::Filter;

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
#[tokio::main]
async fn main() {
    let redis_address =
        env::var("REDIS_ADDRESS").expect("The REDIS_ADDRESS env var must be set at runtime.");

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let client = Client::open(redis_address.clone()).expect(&format!(
        "Unable to open client connection to {}",
        &redis_address
    ));
    let ctx = client
        .get_multiplexed_tokio_connection()
        .await
        .expect("Could not create multiplexed Redis Connection.");

    let api = filters::todos::todos(ctx);

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("todos"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
