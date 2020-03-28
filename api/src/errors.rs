use redis::RedisError;
use serde_json::Error as SerdeError;
use warp::reject::Reject;

#[derive(Debug)]
pub enum ApiError {
    Redis(RedisError),
    Serde(SerdeError),
}

impl Reject for ApiError {}
