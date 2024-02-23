use std::hash::Hash;

use serde::{Deserialize, Serialize};
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Listener {
    pub id: u64,
}

impl Listener {
    pub fn new(id: u64) -> Listener {
        Listener { id }
    }
}