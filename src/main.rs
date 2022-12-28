#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

extern crate dotenv;

use actix_web::{App, HttpServer};
use listenfd::ListenFd;

use std::{collections::HashMap, ptr::null};
mod model;
mod schema;
mod db;
mod routes;
mod error_handler;

use model::NewTodo;
use model::Todo;
use routes::show_todos;
use std::env;
use dotenv::dotenv;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(routes::init_routes));
    
    server = match listenfd.take_tcp_listener(0)?{
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Please set host in .env");
            let port = env::var("PORT").expect("Please set port in .env");
            server.bind(format!("{}:{}", host, port))?
        }
    };
    println!("Listening in port :5000");
    server.run().await
}