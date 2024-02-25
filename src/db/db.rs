use std::hash::Hash;

use crate::models::identifiable::Identifiable;

pub struct NotFoundErr {}

#[allow(async_fn_in_trait)] // TODO maybe remove this and actually implement std::future in the trait functions
pub trait Database {
    async fn save_object<T>(&mut self, obj: &T) -> bool
    where
        T: Hash + serde::Serialize;

    async fn fetch_object<T>(&mut self, obj_id: u128) -> Option<T>
    where
        T: serde::de::DeserializeOwned;

    async fn fetch_list<T>(&mut self, list_id: &str) -> Option<Vec<T>>
    where
        T: serde::de::DeserializeOwned + Identifiable;

    async fn add_to_list<T>(&mut self, list_id: &str, value: &T) -> bool
    where
        T: serde::Serialize;

    async fn fetch_from_list<T>(&mut self, list_id: &str, obj_id: u128) -> Option<T>
    where
        T: serde::de::DeserializeOwned + Identifiable,
    {
        let list = self.fetch_list::<T>(&list_id).await;
        if let Some(list) = list {
            for item in list {
                if item.get_id() == obj_id {
                    return Some(item);
                }
            }
        }
        None
    }
    async fn delete_from_list<T>(&mut self, list_id: &str, obj_id: &T) -> bool
    where
        T: serde::Serialize;

    async fn delete(&mut self, key: &str) -> bool;

    fn deserialize<T>(&self, string_repr: &String) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let result: Result<T, serde_json::Error> = serde_json::from_str(&string_repr);

        match result {
            Ok(obj) => Some(obj),
            Err(_) => None,
        }
    }
    fn serialize<T>(&self, value: &T) -> Option<String>
    where
        T: serde::Serialize,
    {
        let result = serde_json::to_string(value);

        match result {
            Ok(result) => Some(result),
            Err(_) => None,
        }
    }
}
