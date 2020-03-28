use super::handlers;
use super::models::{Context, ListOptions, PartialTodo, Todo};
use uuid::Uuid;
use warp::{Filter, Rejection, Reply};

/// The 4 TODOs filters combined.
pub fn todos(ctx: Context) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    todos_list(ctx.clone())
        .or(todos_create(ctx.clone()))
        .or(todos_update(ctx.clone()))
        .or(todos_delete(ctx))
}

/// GET /todos?offset=3&limit=5
pub fn todos_list(ctx: Context) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(ctx))
        .and_then(handlers::list_todos)
}

/// POST /todos with JSON body
pub fn todos_create(ctx: Context) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos")
        .and(warp::post())
        .and(partial_todo_body())
        .and(with_db(ctx))
        .and_then(handlers::create_todo)
}

/// PUT /todos/:id with JSON body
pub fn todos_update(ctx: Context) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos")
        .and(warp::put())
        .and(todo_body())
        .and(with_db(ctx))
        .and_then(handlers::update_todo)
}

/// DELETE /todos/:id
pub fn todos_delete(ctx: Context) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todos" / Uuid)
        // It is important to put the auth check _after_ the path filters.
        // If we put the auth check before, the request `PUT /todos/invalid-string`
        // would try this filter and reject because the authorization header doesn't match,
        // rather because the param is wrong for that other path.
        .and(warp::delete())
        .and(with_db(ctx))
        .and_then(handlers::delete_todo)
}

fn with_db(
    ctx: Context,
) -> impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || ctx.clone())
}

fn todo_body() -> impl Filter<Extract = (Todo,), Error = Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn partial_todo_body() -> impl Filter<Extract = (PartialTodo,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
