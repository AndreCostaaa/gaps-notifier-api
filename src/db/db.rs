use std::hash::Hash;

pub struct NotFoundErr {}
pub trait Database {
    fn save<T>(&mut self, obj: &T) -> bool
    where
        T: Hash + serde::Serialize;

    fn fetch<T>(&mut self, obj_id: u64) -> Option<T>
    where
        T: serde::de::DeserializeOwned;

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
