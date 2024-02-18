#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use std::env;

mod db;
use crate::db::DbPool;
mod models;
mod schema;
mod todo;

async fn get_todo_handler(pool: web::Data<DbPool>) -> impl Responder {
    match todo::get_todos(pool).await {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_todo_handler(pool: web::Data<DbPool>, new_todo: web::Json<models::NewTodo>) -> impl Responder {
    match todo::create_todo(pool, new_todo).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_todo_handler(pool: web::Data<DbPool>, todo_id: web::Path<i32> ,update_todo: web::Json<models::UpdateTodo>) -> impl Responder {
    match todo::update_todo(pool, todo_id, update_todo).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_todo_handler(pool: web::Data<DbPool>, todo_id: web::Path<i32>) -> impl Responder {
    match todo::delete_todo(pool, todo_id).await {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = db::establish_connection_pool(&database_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/todos", web::get().to(get_todo_handler))
            .route("/todos", web::post().to(create_todo_handler))
            .route("/todos/{id}", web::put().to(update_todo_handler))
            .route("/todos/{id}", web::delete().to(delete_todo_handler))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
