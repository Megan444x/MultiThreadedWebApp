use actix_web::{web, App, HttpServer, middleware, HttpResponse};
use std::env;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

struct AppState {
    db_pool: sqlx::PgPool,
}

async fn index(data: web::Data<AppState>) -> HttpResponse {
    let result = "Hello, world!".to_string(); 
    HttpResponse::Ok().body(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 
    
    let workers = num_cpus::get();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in the .env file");
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create DB pool.");

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                db_pool: db_pool.clone(),
            })
            .wrap(middleware::Logger::default()) 
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .workers(workers) 
    .run()
    .await
}