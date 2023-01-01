use clap::Error;
use diesel::prelude::*;
use serde::Deserialize;
use crate::schema::users::{self, username};
use crate::{schema, helpers};
use serde::Serialize;

use crate::db::{self, db_connection};
use crate::error_handler::CustomError;


#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User{
    pub id: i32,
    pub username: String, 
    pub email: String,
    pub password: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct Users{
    pub username: String, 
    pub email: String, 
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct Login{
    pub username: String,
    pub password: String
}

impl User {
    pub fn sign_up(mut users: Users) -> Result<Self, CustomError>{
        if Self::check_username_already_present(&users.username).is_err(){
            let mut conn = db::db_connection();
            let password = helpers::encode_password(users.password);
            users.password = password;
            let user_signup = diesel::insert_into(users::table).values(users).get_result(&mut conn)?;
            Ok(user_signup)
        }
        else{
            Err(CustomError { error_status_code: 404, error_message: "Couldnot authenticate".to_string() })
        }
    }

    

    pub fn login(login: Login) -> Result<Self, CustomError>{
        use self::schema::users::dsl::users;
        let mut conn = db::db_connection();
        let user_present = Self::check_username_already_present(&login.username);
        if Self::check_username_already_present(&login.username).is_ok() {
            let user = users.filter(username.eq(&login.username)).get_result::<User>(&mut conn)?;
            if helpers::verify_password(&login.password).eq(&user.password){
                return Ok(user);
            }
            else {
                Err(CustomError { error_status_code: 404, error_message: "Couldnot verify user".to_string() })
            }
        }
        else {
            Err(CustomError { error_status_code: 404, error_message: "Couldnot find the user".to_string() })
        }
    }

    pub fn check_username_already_present(un: &str) -> QueryResult<User>{
        use self::schema::users::dsl::users;
        let mut conn = db::db_connection();
        return users.filter(username.eq(un)).get_result(&mut conn)
    }

}