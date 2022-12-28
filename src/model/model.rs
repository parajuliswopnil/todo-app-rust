use clap::Error;
use diesel::prelude::*;
use serde::Deserialize;
use crate::schema::todos;
use serde::Serialize;

use crate::db::{self, db_connection};
use crate::error_handler::CustomError;

#[derive(Serialize, Insertable, Queryable, Deserialize, AsChangeset)]
#[table_name = "todos"]
pub struct NewTodo{
    pub title: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
}

#[derive(Serialize, Deserialize, Debug, Queryable, AsChangeset)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
}

impl Todo {
    pub fn create(todo: NewTodo) -> Result<Self, CustomError>{
        let mut conn = db_connection();
        let todo = diesel::insert_into(todos::table).values(todo).get_result(&mut conn)?;
        Ok(todo)
    }

    pub fn update(todo: Todo) -> Result<Self, CustomError>{
        let mut conn = db_connection();
        let todo = diesel::update(todos::table).filter(todos::id.eq(todo.id)).set(todo).get_result(&mut conn)?;
        Ok(todo)
    }

    pub fn delete(todo: Todo) -> Result<usize, CustomError>{
        let mut conn = db_connection();
        let todo = diesel::delete(todos::table).filter(todos::id.eq(todo.id)).execute(&mut conn)?;
        Ok(todo)
    }

    pub fn get_todo() -> Result<Vec<Self>, CustomError> {
        let mut conn = db_connection();
        let todo = todos::table.load::<Todo>(&mut conn)?;
        Ok(todo)
    }
}

impl NewTodo {
    fn from (todo: NewTodo) -> NewTodo{
        NewTodo {
            title: todo.title,
            description: todo.description,
            start_time: todo.start_time,
            end_time: todo.end_time,
        }
    }
}






