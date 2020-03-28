use redis::RedisError;
use warp::reject::Reject;

#[derive(Debug)]
pub enum ApiError {
    Redis(RedisError),
}

impl Reject for ApiError {}
