use crate::{
    db::db::Database,
    models::{grade::Grade, identifiable::Identifiable, subscriber::Subscriber},
};

fn list_key(subscriber_id: u128) -> String {
    format!("sub_{}", subscriber_id.to_string())
}
pub async fn register<Db: Database>(db: &mut Db, subscriber: &Subscriber) -> bool {
    let list_key = list_key(subscriber.get_id());

    db.add_to_list(&list_key, subscriber).await
}

pub async fn get_subscribers<Db: Database>(db: &mut Db, grade: &Grade) -> Vec<Subscriber> {
    let cl_id = Subscriber::compute_key(&grade.class, &grade.course, grade.year);

    let list_key = list_key(cl_id.into());
    db.fetch_list::<Subscriber>(&list_key)
        .await
        .unwrap_or_default()
}
