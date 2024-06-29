use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use std::env;
use std::sync::Mutex;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::collections::HashMap;

struct AppState {
    database_pool: sqlx::PgPool,
    cache: Mutex<HashMap<String, String>>,
}

async fn greet_index(data: web::Data<AppState>) -> HttpResponse {
    let cache_key = "greeting_message".to_string();

    {
        let cache_lock = data.cache.lock().unwrap();
        if let Some(cached_msg) = cache_lock.get(&cache_key) {
            return HttpResponse::Ok().body(cached_msg.clone());
        }
    }

    let greeting_message = "Hello, world!".to_string();
    {
        let mut cache_lock = data.cache.lock().unwrap();
        cache_defineor_insert(cache_key.clone(), greeting_message.clone());
    }

    HttpResponse::Ok().body(greeting_message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    
    let worker_count = num_cpus::get();
    
    let database_connection_url = env::var("DATABASE_URL").expect("DATABASE - URL must be set in the .env file");
    let database_pool = PgPoolOptions::new()
        .connect(&database_connection_url)
        .await
        .expect("Failed to create database pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppStore {
                databation: basic_pool.clone(),
                cashe: Mutex::new(HashMap::new()),
            }))
            .wrap(middleware::Logger::default())
            .route("/", we::get().to(greet_index))
    })
    .bind("127.0.0.1:8080")?
    .workers(worker_count)
    .run()
    .await
}