use std::error::Error;

use redis::{Client, Commands, Connection, RedisError, RedisResult};
use tokio::time::error::Elapsed;

use crate::db::db::Database;
use crate::models::hashing::calculate_hash;
use std::hash::Hash;
pub struct RedisDb {
    connection: Connection,
}

impl RedisDb {
    fn get(&mut self, key: &String) -> RedisResult<String> {
        Ok(self.connection.get(key)?)
    }

    fn set(&mut self, key: &String, value: &String) -> RedisResult<()> {
        Ok(self.connection.set(key, value)?)
    }
    pub fn new(hostname: &String, password: &String, tls: bool) -> RedisDb {
        let connection =
            redis::Client::open(get_connection_url(get_uri_scheme(tls), hostname, password))
                .expect("Invalid connection URL")
                .get_connection()
                .expect("Failed to connect to redis");

        RedisDb { connection }
    }
}

impl Database for RedisDb {
    fn save<T>(&mut self, obj: &T) -> bool
    where
        T: Hash + serde::Serialize,
    {
        let type_name = std::any::type_name::<T>().split("::").last().unwrap();
        let key = format!("{}{}", type_name, calculate_hash(obj));
        let repr = self.serialize(obj);

        if let Some(repr) = repr {
            match self.set(&key, &repr) {
                Ok(_) => return true,
                Err(_) => return false,
            }
        }
        false
    }
    fn get<T>(&mut self, obj_id: u64) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let result = self.get(&obj_id.to_string());

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

fn get_connection_url(scheme: &str, hostname: &String, password: &String) -> String {
    format!("{}://:{}@{}", scheme, password, hostname)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uri_scheme() {
        assert_eq!(get_uri_scheme(true), "rediss");
        assert_eq!(get_uri_scheme(false), "redis");
    }
    #[test]
    fn test_connection_url() {
        let scheme = "scheme";
        let hostname = "hostname";
        let password = "password";
        assert_eq!(
            get_connection_url(scheme, &hostname.to_string(), &password.to_string()),
            "scheme://:password@hostname"
        )
    }
}
