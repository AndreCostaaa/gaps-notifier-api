use crate::{db::db::Database, models::listener::Listener};

use uuid::Uuid;

static LIST_NAME: &str = "listeners";

pub async fn create_listener<Db: Database>(db: &mut Db) -> Option<Listener> {
    let id = Uuid::new_v4().as_u128();
    let listener = Listener::new(id);

    match db.add_to_list(LIST_NAME, &listener).await {
        true => Some(listener),
        false => None,
    }
}

pub async fn get_listener<Db: Database>(db: &mut Db, id: u128) -> Option<Listener> {
    db.fetch_from_list::<Listener>(LIST_NAME, id).await
}

pub async fn delete_listener<Db: Database>(db: &mut Db, id: u128) -> bool {
    db.delete_from_list(LIST_NAME, &Listener::new(id)).await
}
pub async fn get_listeners<Db: Database>(db: &mut Db) -> Option<Vec<Listener>> {
    db.fetch_list(LIST_NAME).await
}
