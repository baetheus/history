use redis::RedisError;
use warp::reject::{custom, Reject, Rejection};

#[derive(Debug)]
pub enum ApiError {
    Redis(RedisError),
}

impl Reject for ApiError {}

impl From<RedisError> for ApiError {
    fn from(error: RedisError) -> Self {
        ApiError::Redis(error)
    }
}

impl From<ApiError> for Rejection {
    fn from(error: ApiError) -> Self {
        custom(error)
    }
}
