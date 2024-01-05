
use log::{info, warn};
use serde::{Serialize, Deserialize};


//model for user
#[derive(Serialize,Deserialize,sqlx::FromRow, Debug)]
#[allow(non_snake_case)]
pub struct UserModel{
    pub id : i32,
    pub namae : String,
    pub email : String,
    pub password : String,
}


