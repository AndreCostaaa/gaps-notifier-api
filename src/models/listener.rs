use serde::{Deserialize, Serialize};
use std::hash::Hash;

use super::identifiable::Identifiable;

#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Listener {
    pub id: u128,
}

impl Listener {
    pub fn new(id: u128) -> Listener {
        Listener { id }
    }
}
impl Identifiable for Listener {
    fn get_id(&self) -> u128 {
        self.id
    }
}
