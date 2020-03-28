use super::errors::*;

use super::models::*;
use redis::{AsyncCommands, RedisError};
use uuid::Uuid;
use warp::{http::StatusCode, reject::Rejection};

fn map_redis(error: RedisError) -> ApiError {
    ApiError::Redis(error)
}

pub async fn list_todos(_: ListOptions, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    let keys: Vec<String> = ctx.keys("todos:*").await.map_err(map_redis)?;

    if keys.len() == 0 {
        let todos: Vec<Todo> = vec![];
        Ok(warp::reply::json(&todos))
    } else {
        let todos: Vec<Todo> = ctx.get(keys).await.map_err(map_redis)?;
        Ok(warp::reply::json(&todos))
    }
}

pub async fn create_todo(
    todo: PartialTodo,
    mut ctx: Context,
) -> Result<impl warp::Reply, Rejection> {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: todo.text,
        completed: todo.completed,
    };

    log::debug!("create_todo: {:?}", todo);

    ctx.set(&format!("todos:{}", &todo.id), todo.clone())
        .await
        .map_err(map_redis)?;

    Ok(warp::reply::json(&todo))
}

pub async fn update_todo(todo: Todo, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("update_todo: todo={:?}", todo);

    let exists: bool = ctx
        .exists(format!("todos:{}", todo.id))
        .await
        .map_err(map_redis)?;

    if exists {
        ctx.set(&format!("todos:{}", todo.id), todo)
            .await
            .map_err(map_redis)?;

        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_todo(id: Uuid, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("delete_todo: id={}", id);

    let exists: bool = ctx
        .exists(format!("todos:{}", id))
        .await
        .map_err(map_redis)?;

    if exists {
        ctx.del(format!("todos:{}", id)).await.map_err(map_redis)?;
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
