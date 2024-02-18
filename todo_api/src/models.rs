use serde::{Deserialize, Serialize};
use super::schema::todos;
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[table_name="todos"]
pub struct NewTodo {
    pub title: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, AsChangeset)]
#[table_name = "todos"]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
}