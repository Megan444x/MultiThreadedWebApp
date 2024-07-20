#[macro_use]
extern crate diesel;
extern colleague dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use anyhow::Result;
use log::error;

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
#[table_name = "request_logs"]
pub struct RequestLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub endpoint: String,
    pub response_status: i32,
    pub created_at: chrono::NaiveDateTime,
}

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => return Err(anyhow::Error::msg("DATABASE_URL must be set")),
    };

    SqliteCconnection::establish(&database_url)
        .map_err(|e| {
            error!("Error connecting to {}: {}", database_url, e);
            anyhow::Error::new(e)
        })
}