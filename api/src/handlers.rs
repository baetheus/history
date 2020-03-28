use super::errors::*;
/// These are our API handlers, the ends of each filter chain.
/// Notice how thanks to using `Filter::and`, we can define a function
/// with the exact arguments we'd expect from each filter in the chain.
/// No tuples are needed, it's auto flattened for the functions.
use super::models::*;
use redis::{AsyncCommands, RedisError, Value};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reject::{custom, Rejection},
};

pub async fn list_todos(_: ListOptions, ctx: Context) -> Result<impl warp::Reply, Rejection> {
    let keys: Result<Vec<String>, RedisError> = ctx.connection.keys("todos:*").await;
    let keys = match keys {
        Ok(keys) => keys,
        Err(error) => return Err(custom(ApiError::Redis(error))),
    };

    let todos: Result<Vec<Todo>, RedisError> = ctx.connection.get(keys).await;
    match todos {
        Ok(todos) => Ok(warp::reply::json(&todos)),
        Err(error) => Err(custom(ApiError::Redis(error))),
    }
}

pub async fn create_todo(todo: PartialTodo, ctx: Context) -> Result<impl warp::Reply, Rejection> {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: todo.text,
        completed: todo.completed,
    };

    log::debug!("create_todo: {:?}", todo);
    let res: Result<Value, RedisError> = ctx
        .connection
        .set(&format!("todos:{}", todo.id), todo)
        .await;

    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => Err(custom(ApiError::Redis(error))),
    }
}

pub async fn update_todo(todo: Todo, ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("update_todo: todo={:?}", todo);

    let exists: Result<bool, RedisError> =
        ctx.connection.exists(format!("todos:{}", todo.id)).await;

    if let Ok(exists) = exists {
        let res: Result<Value, RedisError> = ctx
            .connection
            .set(&format!("todos:{}", todo.id), todo)
            .await;

        match res {
            Ok(_) => Ok(StatusCode::OK),
            Err(e) => Err(custom(ApiError::Redis(e))),
        }
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_todo(id: Uuid, ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("delete_todo: id={}", id);

    let exists: Result<bool, RedisError> = ctx.connection.exists(format!("todos:{}", id)).await;

    if let Ok(exists) = exists {
        let res: Result<Value, RedisError> = ctx.connection.del(format!("todos:{}", id)).await;
        match res {
            Ok(_) => Ok(StatusCode::NO_CONTENT),
            Err(e) => Err(custom(ApiError::Redis(e))),
        }
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
