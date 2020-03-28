use super::errors::*;

use super::models::*;
use redis::{AsyncCommands, RedisError, Value};
use uuid::Uuid;
use warp::{
    http::StatusCode,
    reject::{custom, Rejection},
};

pub async fn list_todos(_: ListOptions, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    let keys: Result<Vec<String>, RedisError> = ctx.keys("todos:*").await;
    let keys = match keys {
        Ok(keys) => keys,
        Err(error) => return Err(custom(ApiError::Redis(error))),
    };

    if keys.len() == 0 {
        let todos: Vec<Todo> = vec![];
        Ok(warp::reply::json(&todos))
    } else {
        let todos: Result<Vec<Todo>, RedisError> = ctx.get(keys).await;
        match todos {
            Ok(todos) => Ok(warp::reply::json(&todos)),
            Err(error) => Err(custom(ApiError::Redis(error))),
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
    let res: Result<Value, RedisError> = ctx.set(&format!("todos:{}", todo.id), todo).await;

    match res {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(error) => Err(custom(ApiError::Redis(error))),
    }
}

pub async fn update_todo(todo: Todo, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("update_todo: todo={:?}", todo);

    let exists: Result<bool, RedisError> = ctx.exists(format!("todos:{}", todo.id)).await;

    match exists {
        Ok(exists) => {
            if exists {
                let res: Result<Value, RedisError> =
                    ctx.set(&format!("todos:{}", todo.id), todo).await;

                match res {
                    Ok(_) => Ok(StatusCode::OK),
                    Err(e) => Err(custom(ApiError::Redis(e))),
                }
            } else {
                Ok(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => Err(custom(ApiError::Redis(e))),
    }
}

pub async fn delete_todo(id: Uuid, mut ctx: Context) -> Result<impl warp::Reply, Rejection> {
    log::debug!("delete_todo: id={}", id);

    let exists: Result<bool, RedisError> = ctx.exists(format!("todos:{}", id)).await;

    match exists {
        Ok(exists) => {
            if exists {
                let res: Result<Value, RedisError> = ctx.del(format!("todos:{}", id)).await;
                match res {
                    Ok(_) => Ok(StatusCode::NO_CONTENT),
                    Err(e) => Err(custom(ApiError::Redis(e))),
                }
            } else {
                Ok(StatusCode::NOT_FOUND)
            }
        }
        Err(e) => Err(custom(ApiError::Redis(e))),
    }
}
