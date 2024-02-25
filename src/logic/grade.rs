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

    let subscribers = super::subscriber::get_subscribers(db, &grade).await;

    println!("Notifying {} subscribers", subscribers.len());
    for sub in subscribers {
        match discord::events::send_event(&sub.webhook_url, &grade).await {
            //TODO handle errors
            Ok(_) => (),
            Err(e) => {
                println!("Error notifying subscriber: {}", e);
            }
        }
    }
    let spies = super::spy::get_spies(db).await;
    println!("Notifying {} spies", spies.len());
    for spy in spies {
        match discord::events::send_event(&spy.webhook_url, &grade).await {
            //TODO handle errors
            Ok(_) => (),
            Err(e) => {
                println!("Error notifying spy: {}", e);
            }
        }
    }

    result
}
