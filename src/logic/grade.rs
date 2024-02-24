use tracing::info;

use super::{hashing::calculate_hash, time::current_school_year};
use crate::db::db::Database;
use crate::discord;
use crate::models::grade::Grade;

static MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

pub async fn register_grade<Db: Database>(
    db: &mut Db,
    course: String,
    name: String,
    class: String,
    class_average: f64,
) -> bool {
    let guard = MUTEX.lock().await;
    let grade = Grade::new(course, class, current_school_year(), name, class_average);

    let grade_hash: u128 = calculate_hash(&grade).into();

    println!("Grade hash: {}", grade_hash);
    if db.fetch_object::<Grade>(grade_hash).await.is_some() {
        println!("Grade already exists");
        return false;
    }
    println!("Saving grade");
    let result = db.save_object(&grade).await;
    drop(guard);

    let listeners = super::course_listener::get_course_listeners(db, &grade);
    match listeners.await {
        Some(listeners) => {
            println!("Notifying {} listeners", listeners.len());
            for listener in listeners {
                match discord::events::send_event(&listener, &grade).await {
                    //TODO handle errors
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error notifying listener: {}", e);
                    }
                }
            }
        }
        None => {
            info!("No listeners interested in this grade");
        }
    }
    return result;
}
