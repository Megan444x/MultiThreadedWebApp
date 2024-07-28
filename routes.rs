use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

mod handlers;
mod models;

struct AppState {
    users: Mutex<models::Users>,
    request_logs: Mutex<models::RequestLogs>,
}

async fn add_user(data: web::Data<AppState>, user_info: web::Json<models::User>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.add(user_info.into_inner());
    HttpResponse::Created().finish()
}

async fn update_user(data: web::Data<AppState>, user_info: web::Json<models::User>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.update(user_info.into_inner());
    HttpResponse::Ok().finish()
}

async fn delete_user(data: web::Data<AppState>, user_id: web::Path<u32>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.delete(user_id.into_inner());
    HttpResponse::Ok().finish()
}

async fn get_user(data: web::Data<AppState>, user_id: web::Path<u32>) -> impl Responder {
    let users = data.users.lock().unwrap();
    if let Some(user) = users.get(user_id.into_inner()) { 
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn log_request(data: web::Data<AppState>, log_info: web::Json<models::RequestLog>) -> impl Responder {
    let mut request_logs = data.request_logs.lock().unwrap();
    request_logs.log(log_info.into_inner());
    HttpResponse::Created().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        users: Mutex::new(models::Users::new()),
        request_logs: Mutex::new(models::RequestLogs::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/users/add", web::post().to(add_user))
            .route("/users/{id}/update", web::put().to(update_user))
            .route("/users/{id}/delete", web::delete().to(delete_user))
            .route("/users/{id}", web::get().to(get_user))
            .route("/log/request", web::post().to(log_request))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}