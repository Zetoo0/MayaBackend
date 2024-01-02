use log::{info, warn};
use serde::{Serialize, Deserialize};


#[derive(Serialize,Deserialize,sqlx::FromRow, Debug)]
#[allow(non_snake_case)]
pub struct Course{
    pub id : i32,
    pub course_id : String,
    pub  namae : String,
    pub  recommended_semester : i32,
    pub credit : i32,
}

#[derive(Serialize,Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct CourseModelResponse{
    pub id : u8,
    pub course_id : String,
    pub  name : String,
    pub recommended_semester : u8,
    pub credit : u8,
}