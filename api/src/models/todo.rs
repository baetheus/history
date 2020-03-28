use serde_derive::{Deserialize, Serialize};
use serde_json::{from_slice, to_vec};
use uuid::Uuid;

use redis::{ErrorKind, FromRedisValue, RedisResult, RedisWrite, ToRedisArgs, Value};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PartialTodo {
    pub text: String,
    pub completed: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
}

impl FromRedisValue for Todo {
    fn from_redis_value(value: &Value) -> RedisResult<Self> {
        match value {
            Value::Data(v) => {
                let slice: &[u8] = v;
                let result: Todo =
                    from_slice(slice).map_err(|_| (ErrorKind::TypeError, "Parsing Error"))?;
                Ok(result)
            }
            _ => Err((ErrorKind::TypeError, "Incorrect redis value for Todo"))?,
        }
    }
}

impl ToRedisArgs for Todo {
    fn write_redis_args<W: ?Sized>(&self, out: &mut W)
    where
        W: RedisWrite,
    {
        let output: &[u8] = &to_vec(self).expect("Incorrect struct for todo");
        out.write_arg(output);
    }
}
