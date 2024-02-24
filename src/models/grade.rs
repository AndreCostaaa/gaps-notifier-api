use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Serialize, Deserialize)]
pub struct Grade {
    pub course: String,
    pub class: String,
    pub year: i32,
    pub name: String,
    pub class_average: f64,
}

impl Grade {
    pub fn new(
        course: String,
        class: String,
        year: i32,
        name: String,
        class_average: f64,
    ) -> Grade {
        Grade {
            course,
            class,
            year,
            name,
            class_average,
        }
    }
}

impl Hash for Grade {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.course.hash(hasher);
        self.class.hash(hasher);
        self.year.hash(hasher);
        self.name.hash(hasher);
        self.class_average.to_bits().hash(hasher);
    }
}
