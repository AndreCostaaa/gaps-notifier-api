use serde::{Deserialize, Serialize};

use crate::logic;

use super::{identifiable::Identifiable, user::User};

#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Spy {
    pub user: User,
    pub webhook_url: String,
}

impl Identifiable for Spy {
    fn get_id(&self) -> u128 {
        logic::hashing::calculate_hash(self).into()
    }
}

impl Spy {
    pub fn new(user: User, webhook_url: String) -> Spy {
        Spy { user, webhook_url }
    }
}
