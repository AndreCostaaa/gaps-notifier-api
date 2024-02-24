use redis::{aio::ConnectionManager, AsyncCommands, RedisResult};

use crate::db::db::Database;
use crate::logic::hashing::calculate_hash;
use crate::models::identifiable::Identifiable;
use std::hash::Hash;

#[derive(Clone)]
pub struct RedisDb {
    connection: ConnectionManager,
}

impl RedisDb {
    async fn get(&mut self, key: &String) -> RedisResult<String> {
        Ok(self.connection.get(key).await?)
    }

    // async fn set(&mut self, key: &String, value: &String) -> RedisResult<()> {
    //     Ok(self.connection.set(key, value).await?)
    // }
    async fn set_nx(&mut self, key: &String, value: &String) -> RedisResult<()> {
        Ok(self.connection.set_nx(key, value).await?)
    }
    async fn list_append(&mut self, key: &str, value: &str) -> RedisResult<()> {
        Ok(self.connection.rpush(key, value).await?)
    }
    async fn get_list(&mut self, key: &str) -> RedisResult<Vec<String>> {
        Ok(self.connection.lrange(key, 0, -1).await?)
    }
    async fn delete_key(&mut self, key: &str) -> RedisResult<()> {
        Ok(self.connection.del(key).await?)
    }
    async fn delete_from_list(&mut self, list_id: &str, obj_id: &str) -> RedisResult<()> {
        Ok(self.connection.lrem(list_id, 0, &obj_id).await?)
    }
    fn compute_object_key<T>(obj: &T) -> String
    where
        T: Hash,
    {
        Self::compute_obj_key::<T>(calculate_hash(obj).into())
    }
    fn compute_obj_key<T>(obj_id: u128) -> String {
        format!(
            "{}_{}",
            std::any::type_name::<T>().split("::").last().unwrap(),
            obj_id
        )
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
    async fn save_object<T>(&mut self, obj: &T) -> bool
    where
        T: Hash + serde::Serialize,
    {
        let key = Self::compute_object_key(obj);
        let repr = self.serialize(obj);
        if let Some(repr) = repr {
            match self.set_nx(&key, &repr).await {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
        false
    }
    async fn fetch_object<T>(&mut self, obj_id: u128) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let key = Self::compute_obj_key::<T>(obj_id);
        let result = self.get(&key).await;

        if let Ok(result) = result {
            self.deserialize(&result)
        } else {
            None
        }
    }

    async fn fetch_list<T>(&mut self, list_id: &str) -> Option<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Identifiable,
    {
        let result = self.get_list(list_id).await;

        if let Ok(result) = result {
            let mut vec = Vec::new();
            for item in result {
                if let Some(deserialized) = self.deserialize(&item) {
                    vec.push(deserialized);
                }
            }
            Some(vec)
        } else {
            None
        }
    }
    async fn add_to_list<T>(&mut self, list_id: &str, value: &T) -> bool
    where
        T: serde::Serialize,
    {
        let repr = self.serialize(value);
        if let Some(repr) = repr {
            match self.list_append(list_id, &repr).await {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
        false
    }

    async fn delete(&mut self, key: &str) -> bool {
        match self.delete_key(key).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    async fn delete_from_list<T>(&mut self, list_id: &str, obj: &T) -> bool
    where
        T: serde::Serialize,
    {
        let repr = self.serialize(&obj);
        if let Some(repr) = repr {
            match self.delete_from_list(&list_id, &repr).await {
                Ok(_) => true,
                Err(_) => false,
            }
        } else {
            false
        }
    }
}
