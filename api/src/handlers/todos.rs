use crate::models::{context::*, error::ApiError::Redis, list_options::*, todo::*};
use redis::AsyncCommands;
use uuid::Uuid;
use warp::{http::StatusCode, reject::Rejection};

pub async fn list_todos(
    opts: ListOptions,
    mut ctx: Context,
) -> Result<impl warp::Reply, Rejection> {
    let keys: Vec<String> = ctx.keys("todos:*").await.map_err(Redis)?;
    let keys: Vec<String> = keys
        .into_iter()
        .skip(opts.offset.unwrap_or(0))
        .take(opts.limit.unwrap_or(std::usize::MAX))
        .collect();

    match &keys.len() {
        0 => {
            let todos: Vec<Todo> = vec![];
            Ok(warp::reply::json(&todos))
        }
        1 => {
            let todo: Todo = ctx.get(keys).await.map_err(Redis)?;
            let todos = vec![todo];
            Ok(warp::reply::json(&todos))
        }
        _ => {
            let todos: Vec<Todo> = ctx.get(keys).await.map_err(Redis)?;
            Ok(warp::reply::json(&todos))
        }
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
        .map_err(Redis)?;

    Ok(warp::reply::json(&todo))
}

pub async fn update_todo(todo: Todo, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("update_todo: todo={:?}", todo);

    let exists: bool = ctx
        .exists(format!("todos:{}", todo.id))
        .await
        .map_err(Redis)?;

    if exists {
        ctx.set(&format!("todos:{}", todo.id), todo)
            .await
            .map_err(Redis)?;

        Ok(StatusCode::OK)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}

pub async fn delete_todo(id: Uuid, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("delete_todo: id={}", id);

    let exists: bool = ctx.exists(format!("todos:{}", id)).await.map_err(Redis)?;

    if exists {
        ctx.del(format!("todos:{}", id)).await.map_err(Redis)?;
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}
