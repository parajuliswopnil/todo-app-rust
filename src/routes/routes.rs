use crate::model::{Todo, NewTodo};
use crate::db::db_connection;

use crate::error_handler::CustomError;
use actix_web::web::to;
use clap::Error;
use diesel::prelude::*;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[post("/create")]
async fn create(todo: web::Json<NewTodo>) -> Result<HttpResponse, CustomError>{
    println!("reached here");
    let todo = Todo::create(todo.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[post("/update/{id}")]
async fn update(todo: web::Json<Todo>) -> Result<HttpResponse, CustomError>{
    println!("updating todo");
    let todo = Todo::update(todo.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[post("/delete")]
async fn delete(todo: web::Json<Todo>) -> Result<HttpResponse, CustomError>{
    println!("deleting todo");
    let todo = Todo::delete(todo.into_inner())?;
    Ok(HttpResponse::Ok().json(todo))
}

#[get("/todos")]
async fn show_todos() -> Result<HttpResponse, CustomError>{
    println!("Showing todos");
    let todo = Todo::get_todo()?;
    Ok(HttpResponse::Ok().json(todo))
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(show_todos);
}