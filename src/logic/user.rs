use crate::{db::db::Database, models::user::User};

use uuid::Uuid;

static LIST_NAME: &str = "users";

pub async fn create_user<Db: Database>(db: &mut Db) -> Option<User> {
    let id = Uuid::new_v4().as_u128();
    let user = User::new(id);

    match db.add_to_list(LIST_NAME, &user).await {
        true => Some(user),
        false => None,
    }
}

pub async fn get_user<Db: Database>(db: &mut Db, id: u128) -> Option<User> {
    db.fetch_from_list(LIST_NAME, id).await
}

pub async fn get_users<Db: Database>(db: &mut Db) -> Option<Vec<User>> {
    db.fetch_list(LIST_NAME).await
}
