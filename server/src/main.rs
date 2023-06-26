use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use server::*;

use common;


#[get("/api/auth/register")]
async fn register(data: web::Data<AppState>, info: web::Json<common::schemas::UserRegister>) -> impl Responder {
    let user = database::check_user_exists(
        &data.db_pool, 
        &info.into_inner().username)
        .await;
    match user {
        Some(x) => x.username.clone(),
        None => "no"
    }
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let env_vars = get_env().await;

    let db_pool = database::initialize_connection(&env_vars.db_url).await; 
    
    let opts = AppState {
        db_pool,
        env_vars,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(opts.clone()))
            .service(register)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8070))?
    .run()
    .await
}



