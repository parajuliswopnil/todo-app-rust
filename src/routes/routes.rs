use crate::model::{Todo, NewTodo};
use crate::db::db_connection;
use diesel::prelude::*;

pub fn create_todo(todo: NewTodo){
    use crate::schema::todos::dsl::*;

    let mut connection = db_connection();

    let new_todo = NewTodo{
        title: &todo.title,
        description: &todo.description,
        start_time: todo.start_time,
        end_time: todo.end_time
    };

    diesel::insert_into(todos).values(&new_todo).execute(&mut connection).expect("Error in saving todo");
}

pub fn update_todo(todo: Todo){
    use crate::schema::todos::dsl::*;
    let mut connection = db_connection();

    let db_todo = Todo{
        id: todo.id,
        title: todo.title,
        description: todo.description,
        start_time: todo.start_time,
        end_time: todo.end_time,
    };

    diesel::update(todos.find(todo.id)).set(&db_todo).execute(&mut connection).expect("Error in updating todo");
}

pub fn show_todos(){
    println!("Showing todos");
    use crate::schema::todos::dsl::*;

    let mut connection = db_connection();
    let results = todos.load::<Todo>(&mut connection).expect("Error loading videos");
    for video in results{
        println!("{:?}", video);
    }
}