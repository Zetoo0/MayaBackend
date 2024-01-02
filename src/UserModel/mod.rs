
use log::{info, warn};
use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,sqlx::FromRow, Debug)]
#[allow(non_snake_case)]
pub struct UserModel{
    pub id : i32,
    pub neptun_id : String,
    pub maya_id : String,
    pub namae : String,
    pub email : String,
    pub password : String,
    pub user_logname : String
}


