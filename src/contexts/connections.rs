use redis::{aio::Connection, AsyncCommands, IntoConnectionInfo, RedisError, RedisResult};
use std::sync::Arc;
use teloxide::types::User;
use tokio::sync::Mutex;

pub struct ConnectionContext {
    redis_connection: Mutex<Connection>,
    ans: String,
}

impl ConnectionContext {
    pub async fn open(redis_url: impl IntoConnectionInfo) -> Result<Arc<Self>, RedisError> {
        Ok(Arc::new(Self {
            redis_connection: Mutex::new(
                redis::Client::open(redis_url)?
                    .get_async_connection()
                    .await?,
            ),
            ans: String::new(),
        }))
    }
    pub fn set_ans(&mut self, ans: String) {
        self.ans = ans;
    }
    pub fn get_ans_clone(&self) -> String {
        self.ans.clone()
    }
    pub async fn add_user_presence(self: Arc<Self>, user: &User) -> Result<(), RedisError> {
        self.redis_connection
            .lock()
            .await
            .sadd(
                "present",
                user.clone()
                    .username
                    .unwrap_or("usuario_random".to_string()),
            )
            .await
    }
    pub async fn delete_user_presence(self: Arc<Self>, user: &User) -> Result<(), RedisError> {
        self.redis_connection
            .lock()
            .await
            .srem(
                "present",
                user.clone()
                    .username
                    .unwrap_or("usuario_random".to_string()),
            )
            .await
    }
    pub async fn get_presence_count(self: Arc<Self>) -> Result<u32, RedisError> {
        self.redis_connection.lock().await.scard("present").await
    }
}
