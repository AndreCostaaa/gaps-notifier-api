use crate::{db::db::Database, models::spy::Spy};

static LIST_NAME: &str = "spies";

pub async fn register_spy<Db: Database>(db: &mut Db, spy: &Spy) -> bool {
    db.add_to_list(LIST_NAME, spy).await
}

pub async fn delete_spy<Db: Database>(db: &mut Db, spy: &Spy) -> bool {
    db.delete_from_list(LIST_NAME, &spy).await
}

pub async fn get_spies<Db: Database>(db: &mut Db) -> Vec<Spy> {
    db.fetch_list(LIST_NAME).await.unwrap_or_default()
}
