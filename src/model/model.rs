use diesel::prelude::*;
use crate::schema::todos;

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo<'a>{
    pub title: &'a str,
    pub description: &'a str,
    pub start_time: i32,
    pub end_time: i32,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub start_time: i32,
    pub end_time: i32,
}




