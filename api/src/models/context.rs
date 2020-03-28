use redis::aio::MultiplexedConnection;

pub type Context = MultiplexedConnection;
