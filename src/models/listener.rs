use serde::{Deserialize, Serialize};
use std::hash::Hash;
use uuid::Uuid;
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Listener {
    pub id: u128,
}

impl Listener {
    pub fn new(id: u128) -> Listener {
        Listener { id }
    }
    pub fn new_with_random_uuid() -> Listener {
        Listener {
            id: Uuid::new_v4().as_u128(),
        }
    }
}
