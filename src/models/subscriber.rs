use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use super::{identifiable::Identifiable, user::User};
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct Subscriber {
    pub user: User,
    pub course: String,
    pub class: String,
    pub year: i32,
    pub webhook_url: String,
}

impl Identifiable for Subscriber {
    fn get_id(&self) -> u128 {
        Subscriber::compute_key(&self.class, &self.course, self.year).into()
    }
}

impl Subscriber {
    pub fn new(
        user: User,
        course: String,
        class: String,
        year: i32,
        webhook_url: String,
    ) -> Subscriber {
        Subscriber {
            user,
            course,
            class,
            year,
            webhook_url,
        }
    }
    pub fn compute_key(class: &String, course: &String, year: i32) -> u64 {
        let mut hasher = DefaultHasher::new();
        class.hash(&mut hasher);
        course.hash(&mut hasher);
        year.hash(&mut hasher);
        hasher.finish()
    }
}
