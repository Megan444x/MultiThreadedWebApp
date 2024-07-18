#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod schema {
    table! {
        users (id) {
            id -> Integer,
            username -> Text,
            email -> Text,
            created_at -> Timestamp,
        }
    }

    table! {
        request_logs (id) {
            id -> Integer,
            user_id -> Nullable<Integer>,
            endpoint -> Text,
            response_status -> Integer,
            created_at -> Timestamp,
        }
    }

    joinable!(request_logs -> users (user_id));
    allow_tables_to_appear_in_same_query!(
        users,
        request_logs,
    );
}

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Associations)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Associations)]
#[belongs_to(User)]
#[table_both_name = "request_logs"]
pub struct RequestLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub endpoint: String,
    pub response_status: i32,
    pub created_at: chrono::NaiveDateTime,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}