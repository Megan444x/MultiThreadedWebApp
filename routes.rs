use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod handlers;
mod models;

struct AppState {
    user_store: Mutex<models::Users>,
    request_log_store: Mutex<models::RequestLogs>,
}

async fn create_user_endpoint(user_data: web::Data<AppState>, user_payload: web::Json<models::User>) -> impl Responder {
    let mut user_store = user_data.user_store.lock().unwrap();
    user_store.add(user_payload.into_inner());
    HttpResponse::Created().finish()
}

async fn update_user_endpoint(user_data: web::Data<AppState>, user_payload: web::Json<models::User>) -> impl Responder {
    let mut user_store = user_data.user_store.lock().unwrap();
    user_store.update(user_payload.into_inner());
    HttpResponse::Ok().finish()
}

async fn delete_user_endpoint(user_data: web::Data<AppState>, user_id_path: web::Path<u32>) -> impl Responder {
    let mut user_store = user_data.user_store.lock().unwrap();
    user_store.delete(user_id_path.into_inner());
    HttpResponse::Ok().finish()
}

async fn retrieve_user_endpoint(user_data: web::Data<AppState>, user_id_path: web::Path<u32>) -> impl Responder {
    let user_store = user_data.user_store.lock().unwrap();
    if let Some(user) = user_store.get(user_id_path.into_inner()) { 
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn create_request_log_endpoint(log_data: web::Data<AppState>, log_payload: web::Json<models::RequestLog>) -> impl Responder {
    let mut request_log_store = log_data.request_log_store.lock().unwrap();
    request_log_store.log(log_payload.into_inner());
    HttpResponse::Created().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        user_store: Mutex::new(models::Users::new()),
        request_log_store: Mutex::new(models::RequestLogs::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/users/add", web::post().to(create_user_endpoint))
            .route("/users/{id}/update", web::put().to(update_user_endpoint))
            .route("/users/{id}/delete", web::delete().to(delete_user_endpoint))
            .route("/users/{id}", web::get().to(retrieve_user_endpoint))
            .route("/log/request", web::post().to(create_request_log_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}