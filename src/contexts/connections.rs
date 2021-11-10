use redis::aio::Connection;
use std::sync::Arc;

#[derive(Clone)]
pub struct ConnectionContext {
    redis_connection: Arc<Connection>,
    ans: String,
}

impl ConnectionContext {
    pub fn new(redis_connection: Connection) -> ConnectionContext {
        Self {
            redis_connection: Arc::new(redis_connection),
            ans: String::new(),
        }
    }
    pub fn set_ans(&mut self, ans: String) {
        self.ans = ans;
    }
    pub fn set_redis_connection(&mut self, redis_connection: Arc<Connection>) {
        self.redis_connection = redis_connection;
    }
    pub fn get_ans_clone(&self) -> String {
        self.ans.clone()
    }
    pub fn get_ans_as_ref(&self) -> &str {
        self.ans.as_str()
    }
    pub fn get_redis_connection_clone(&self) -> Arc<Connection> {
        self.redis_connection.clone()
    }
    pub fn get_redis_connection_as_ref(&self) -> &Connection {
        self.redis_connection.as_ref()
    }
    pub fn get_redis_connection_as_mut_ref(&mut self) -> &mut Connection {
        self.redis_connection.as_ref()
    }
}
