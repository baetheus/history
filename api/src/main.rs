mod errors;
mod filters;
mod handlers;
mod models;

use std::{env, sync::Arc};
use warp::Filter;

/// Provides a RESTful web server managing some Todos.
///
/// API will be:
///
/// - `GET /todos`: return a JSON list of Todos.
/// - `POST /todos`: create a new Todo.
/// - `PUT /todos/:id`: update a specific Todo.
/// - `DELETE /todos/:id`: delete a specific Todo.
#[tokio::main]
async fn main() {
    let redis_address =
        env::var("REDIS_ADDRESS").expect("The REDIS_ADDRESS env var must be set at runtime.");

    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let client = redis::Client::open(redis_address.clone()).expect(&format!(
        "Unable to open client connection to {}",
        &redis_address
    ));
    let connection = Arc::new(
        client
            .get_async_connection()
            .await
            .expect("Unable to get async redis client connection."),
    );
    let context = models::Context { connection };

    let api = filters::todos(context);

    // View access logs by setting `RUST_LOG=todos`.
    let routes = api.with(warp::log("todos"));
    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
