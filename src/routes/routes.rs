use crate::model::{Todo, NewTodo};
use crate::model::{User, Users, Login};
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

#[post("/signup")]
async fn sign_up(users: web::Json<Users>) -> Result<HttpResponse, CustomError> {
    println!("Signing you up");
    let user = User::sign_up(users.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login(users: web::Json<Login>) -> Result<HttpResponse, CustomError>{
    println!("Logging you on");
    let user = User::login(users.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(config: &mut web::ServiceConfig){
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(show_todos);
    config.service(sign_up);
    config.service(login);
}
