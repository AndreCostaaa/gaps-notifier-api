use chrono::{Datelike, Utc};
pub fn current_school_year() -> i32 {
    let now = Utc::now();
    let year = now.year();
    let month = now.month();
    if month < 8 {
        year - 1
    } else {
        year
    }
}
