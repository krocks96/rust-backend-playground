use diesel::prelude::*;
use chrono::Utc;
use crate::models::{Todo, NewTodo, UpdateTodo};
use crate::schema::todos::dsl::*;
use actix_web::{web, error, Error};
use crate::db::DbPool;

pub async fn get_todos(pool: web::Data<DbPool>) -> Result<Vec<Todo>, Error> {
    let conn = pool.get().map_err(|_| error::ErrorInternalServerError("Failed to get db connection from pool"))?;
    web::block(move || todos.load::<Todo>(&conn))
        .await
        .map_err(|_| error::ErrorInternalServerError("Error loading todos"))
        .and_then(|res| res.map_err(|_| error::ErrorInternalServerError("Error loading todos")))
}

pub async fn create_todo(
    pool: web::Data<DbPool>,
    new_todo: web::Json<NewTodo>,
) -> Result<Todo, Error> {
    let conn = pool.get().map_err(|_| error::ErrorInternalServerError("Failed to get db connection from pool"))?;

    web::block(move ||
        diesel::insert_into(todos)
            .values(&*new_todo)
            .get_result::<Todo>(&conn)
    )
    .await
    .map_err(|_| error::ErrorInternalServerError("Error creating todo"))
    .and_then(|res| res.map_err(|_| error::ErrorInternalServerError("Error loading todos")))
}

pub async fn update_todo(
    pool: web::Data<DbPool>,
    todo_id: web::Path<i32>,
    update_todo: web::Json<UpdateTodo>,
) -> Result<Todo, Error> {
    let conn = pool.get().map_err(|_| error::ErrorInternalServerError("Failed to get db connection from pool"))?;

    web::block(move ||
        diesel::update(todos.find(*todo_id))
            .set((
                update_todo.into_inner(),
                updated_at.eq(Utc::now().naive_utc()), // `updated_at`を現在のタイムスタンプに設定
            ))
            .get_result::<Todo>(&conn)
    )
    .await
    .map_err(|_| error::ErrorInternalServerError("Error update todo"))
    .and_then(|res| res.map_err(|_| error::ErrorInternalServerError("Error loading todos")))
}

pub async fn delete_todo(
    pool: web::Data<DbPool>,
    todo_id: web::Path<i32>,
) -> Result<Todo, Error> {
    let conn = pool.get().map_err(|_| error::ErrorInternalServerError("Failed to get db connection from pool"))?;

    web::block(move ||
        diesel::delete(todos.find(*todo_id))
            .get_result::<Todo>(&conn)
    )
    .await
    .map_err(|_| error::ErrorInternalServerError("Error update todo"))
    .and_then(|res| res.map_err(|_| error::ErrorInternalServerError("Error loading todos")))
}