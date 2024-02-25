use serde::{Deserialize, Serialize};
use std::hash::Hash;

use super::identifiable::Identifiable;

#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u128,
}

impl User {
    pub fn new(id: u128) -> User {
        User { id }
    }
}
impl Identifiable for User {
    fn get_id(&self) -> u128 {
        self.id
    }
}
