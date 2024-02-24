/**
 *
 * File: listener.rs
 * Date: 2021-07-07
 * Author: André Costa
 * A listener defines a
 */
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use super::listener::Listener;
#[derive(Hash, Debug, Serialize, Deserialize)]
pub struct CourseListener {
    pub listener: Listener,
    pub course: String,
    pub class: String,
    pub year: u32,
    pub webhook_url: String,
}

impl CourseListener {
    pub fn new(
        listener: Listener,
        course: String,
        class: String,
        year: u32,
        webhook_url: String,
    ) -> CourseListener {
        CourseListener {
            listener,
            course,
            class,
            year,
            webhook_url,
        }
    }
    pub fn compute_key(class: &String, course: &String, year: u32) -> u64 {
        let mut hasher = DefaultHasher::new();
        class.hash(&mut hasher);
        course.hash(&mut hasher);
        year.hash(&mut hasher);
        hasher.finish()
    }
}
