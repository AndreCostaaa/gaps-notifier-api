use crate::{
    db::db::Database,
    models::{course_listener::CourseListener, identifiable::Identifiable},
};

fn list_key(course_listener: &CourseListener) -> String {
    format!("cl_{}", course_listener.get_id())
}
pub async fn register<Db: Database>(db: &mut Db, course_listener: &CourseListener) -> bool {
    let list_key = list_key(&course_listener);

    db.add_to_list(&list_key, course_listener).await
}
