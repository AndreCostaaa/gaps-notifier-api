use crate::db::redis::RedisDb;
#[derive(Clone)]
pub struct ApiState {
    pub redis_db: RedisDb,
}
