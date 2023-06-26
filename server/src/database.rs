use sqlx::{SqlitePool, Pool, Sqlite, FromRow};
use chrono::prelude::*;
use crate::{log, log_err};
use serde::{
    Serialize, Deserialize
};


#[derive(Serialize, Deserialize, Default, FromRow)]
pub struct UserDB {
    pub id: i64,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub token: String,
    pub login_time: DateTime<Utc>,
}


pub async fn initialize_connection(url: &str) -> Pool<Sqlite> {
    let conn = SqlitePool::connect(url);
    match conn.await {
        Ok(x) => {
            log("connection with DB Pool initialized");
            x
        },
        Err(e) => {
            log_err("Couldn't open database");
            panic!("{}", e);
        }
    }
}

pub async fn create_database() {

}

pub async fn check_user_exists(conn: &Pool<Sqlite>, username: &str) -> Option<UserDB> {
    let q = "SELECT * FROM users WHERE username = $1";
    let query = sqlx::query_as::<_, UserDB>(q)
        .bind(username);
    let result = query.fetch_optional(conn).await;
    let result: Option<UserDB> = match result {
        Ok(x) => x,
        Err(_) => None
    };
    result
}
