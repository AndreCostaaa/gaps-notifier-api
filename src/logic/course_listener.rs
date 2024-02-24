use crate::{
    db::db::Database,
    models::{course_listener::CourseListener, grade::Grade, identifiable::Identifiable},
};

fn list_key(course_listener_id: u128) -> String {
    format!("cl_{}", course_listener_id.to_string())
}
pub async fn register<Db: Database>(db: &mut Db, course_listener: &CourseListener) -> bool {
    let list_key = list_key(course_listener.get_id());

    db.add_to_list(&list_key, course_listener).await
}

pub async fn get_course_listeners<Db: Database>(
    db: &mut Db,
    grade: &Grade,
) -> Option<Vec<CourseListener>> {
    let cl_id = CourseListener::compute_key(&grade.class, &grade.course, grade.year);

    let list_key = list_key(cl_id.into());
    db.fetch_list(&list_key).await
}
