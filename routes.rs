use actix::{Actor, SyncContext, Message, ActorContext, SyncArbiter, Handler as MessageHandler};
use actix_web::{web, App, HttpResponse, HttpServer, Responder, error, Error};
use std::sync::RwLock;

mod handlers;
mod models;

struct AppState {
    user_store: RwLock<models::Users>,
    request_log_store: RwLock<models::RequestLogs>,
}

#[derive(Message)]
#[rtype(result = "Result<(), String>")] // Using String to communicate error details
struct CreateUser(models::User);

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
struct UpdateUser(models::User);

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
struct DeleteUser(u32);

#[derive(Message)]
#[rtype(result = "Result<models::User, String>")]
struct GetUser(u32);

#[derive(Message)]
#[rtype(result = "Result<(), String>")]
struct LogRequest(models::RequestLog);

struct DbActor {
    user_store: RwLock<models::Users>,
    request_log_store: RwLock<models::RequestLogs>,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl MessageHandler<CreateUser> for DbActor {
    type Result = Result<(), String>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut user_store = self.user_store.write().map_err(|e| format!("Failed to acquire write lock: {}", e))?;
        user_store.add(msg.0);
        Ok(())
    }
}

async fn create_user_endpoint(db: web::Data<DbActor>, user_payload: web::Json<models::User>) -> Result<HttpResponse, Error> {
    db.send(CreateUser(user_payload.into_inner())).await.map_err(|e| error::ErrorInternalServerError(e))?
        .map_err(|e| error::ErrorBadRequest(e))?;
    Ok(HttpResponse::Created().finish())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = AppState {
        user_store: RwLock::new(models::Users::new()),
        request_log_store: RwLock::new(models::RequestLogs::new()),
    };

    let db_actor = SyncArbiter::start(4, move || DbActor { 
        user_store: data.user_store.clone(), 
        request_log_store: data.request_log_store.clone(),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_actor.clone()))
            .route("/users/add", web::post().to(create_user_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}