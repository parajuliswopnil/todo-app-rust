#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{collections::HashMap, ptr::null};
mod model;
mod schema;
mod db;
mod routes;

use model::NewTodo;
use model::Todo;
use routes::create_todo;
use routes::update_todo;
use routes::show_todos;



fn main() {

    // let todo = NewTodo{
    //     title: "Todo 1",
    //     description: "Description",
    //     start_time: 0,
    //     end_time: 1,
    // };

    // create_todo(todo);
    show_todos();

    let todos = Todo{
        id: 1,
        title: "Todo 2 updated".to_string(),
        description: "Description 2 updated".to_string(),
        start_time: 0,
        end_time: 1,
    };
    update_todo(todos);
    show_todos();
}