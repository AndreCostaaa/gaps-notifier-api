use super::{hashing::calculate_hash, time::current_school_year};
use crate::db::db::Database;
use crate::models::grade::Grade;

pub fn register_grade<Db: Database>(
    db: &mut Db,
    course: String,
    name: String,
    class: String,
    class_average: f64,
) -> bool {
    let grade = Grade::new(course, class, current_school_year(), name, class_average);

    let grade_hash = calculate_hash(&grade);

    if db.fetch::<Grade>(grade_hash.into()).is_some() {
        return false;
    }

    return db.save(&grade);
}
