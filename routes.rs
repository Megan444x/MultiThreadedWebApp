use actix::{Actor, SyncContext, Message, ActorContext, SyncArbiter};
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::RwLock;

mod handlers;
mod models;

struct AppState {
    user_store: RwLock<models::Users>,
    request_log_store: RwLock<models::RequestLogs>,
}

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
struct CreateUser(models::User);

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
struct UpdateUser(models::User);

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
struct DeleteUser(u32);

#[derive(Message)]
#[rtype(result = "Result<models::User, ()>")]
struct GetUser(u32);

#[derive(Message)]
#[rtype(result = "Result<(), ()>")]
struct LogRequest(models::RequestLog);

struct DbActor {
    user_store: RwLock<models::Users>,
    request_log_store: RwLock<models::RequestLogs>,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl MessageHandler<CreateUser> for DbActor {
    type Result = Result<(), ()>;

    fn handle(&mut self, msg: CreateUser, _ctx: &mut Self::Context) -> Self::Result {
        let mut user_store = self.user_store.write().unwrap();
        user_store.add(msg.0);
        Ok(())
    }
}

async fn create_user_endpoint(db: web::Data<DbActor>, user_payload: web::Json<models::User>) -> impl Responder {
    db.send(CreateUser(user_payload.into_inner())).await.unwrap();
    HttpResponse::Created().finish()
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