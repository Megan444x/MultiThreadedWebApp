#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;
use anyhow::{Result, Error};
use log::{error, info};
use chrono::NaiveDateTime;

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
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Insertable, AsChangeset, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "request_logs"]
pub struct RequestLog {
    pub id: i32,
    pub user_id: Option<i32>,
    pub endpoint: String,
    pub response_status: i32,
    pub created_at: NaiveDateTime,
}

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .map_err(|_| Error::msg("DATABASE_URL must be set"))?;

    match SqliteConnection::establish(&database_url) {
        Ok(conn) => {
            info!("Database connection established successfully.");
            Ok(conn)
        },
        Err(e) => {
            error!("Error connecting to {}: {}", database_url, e);
            Err(Error::new(e))
        }
    }
}

pub fn create_test_user(conn: &SqliteConnection) -> Result<usize> {
    use schema::users::dsl::*;
    use diesel::insert_into;

    let new_user = User {
        id: 0, // Auto-increment should handle `id`, would adjust accordingly based on your schema setup.
        username: "testuser".into(),
        email: "test@example.com".into(),
        created_at: chrono::Utc::now().naive_utc(),
    };

    insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(|e| {
            error!("Error creating test user: {}", e);
            e.into()
        })
}

pub fn log_request(conn: &SqliteConnection, user_id: Option<i32>, endpoint: &str, response_status: i32) -> Result<usize> {
    use schema::request_logs::dsl::*;
    use diesel::insert_into;

    let new_log = RequestLog {
        id: 0, // Similarly handled by auto-increment
        user_id,
        endpoint: endpoint.into(),
        response_status,
        created_at: chrono::Utc::now().naive_utc(),
    };

    insert_into(request_logs)
        .values(&new_log)
        .execute(conn)
        .map_err(|e| {
            error!("Error logging request: {}", e);
            e.into()
        })
}