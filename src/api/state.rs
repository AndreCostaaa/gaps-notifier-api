use crate::db::redis::RedisDb;
pub struct ApiState {
    pub redis_db: RedisDb,
}
