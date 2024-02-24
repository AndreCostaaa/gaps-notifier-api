use redis::{aio::ConnectionManager, AsyncCommands, RedisResult};

use crate::db::db::Database;
use crate::logic::hashing::calculate_hash;
use std::hash::Hash;

#[derive(Clone)]
pub struct RedisDb {
    connection: ConnectionManager,
}

impl RedisDb {
    async fn get(&mut self, key: &String) -> RedisResult<String> {
        Ok(self.connection.get(key).await?)
    }

    async fn set(&mut self, key: &String, value: &String) -> RedisResult<()> {
        Ok(self.connection.set(key, value).await?)
    }
    pub async fn new(url: &String) -> RedisDb {
        let connection_manager = redis::Client::open(url.as_str())
            .expect("Invalid connection URL")
            .get_connection_manager()
            .await
            .expect("Failed to create connection manager");

        RedisDb {
            connection: connection_manager,
        }
    }
}

impl Database for RedisDb {
    async fn save<T>(&mut self, obj: &T) -> bool
    where
        T: Hash + serde::Serialize,
    {
        let type_name = std::any::type_name::<T>().split("::").last().unwrap();
        let key = format!("{}{}", type_name, calculate_hash(obj));
        let repr = self.serialize(obj);

        if let Some(repr) = repr {
            match self.set(&key, &repr).await {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
        false
    }
    async fn fetch<T>(&mut self, obj_id: u128) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let result = self.get(&obj_id.to_string()).await;

        if let Ok(result) = result {
            self.deserialize(&result)
        } else {
            None
        }
    }
}
fn get_uri_scheme(tls: bool) -> &'static str {
    match tls {
        true => "rediss",
        false => "redis",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_scheme() {
        assert_eq!(get_uri_scheme(true), "rediss");
        assert_eq!(get_uri_scheme(false), "redis");
    }
}
