use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize, Debug)]
pub struct CreateCourseSchema{
    pub id : u8,
    pub course_id : String,
    pub  name : String,
    pub recommended_semester : u8,
    pub credit : u8
}

#[derive(Serialize,Deserialize, Debug)]
pub struct UpdateCourseSchema{
    pub id : Option<u8>,
    pub course_id : Option<String>,
    pub  name : Option<String>,
    pub recommended_semester : Option<u8>,
    pub credit : Option<u8>
}

