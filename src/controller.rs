pub mod controller{

use std::ptr::null;

use crate::{
    CourseModel::{Course, CourseModelResponse},
    CourseSchema::{CreateCourseSchema, UpdateCourseSchema},
    UserModel::UserModel,
    service,
    AppState,
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;


//---------------------------------------------COURSE RELATED-------------------------------------------------
//get request to gett all courses
#[get("/courses")]
pub async fn getCourses(data : web::Data<AppState>)->impl Responder{


    let courses: Vec<Course> = service::service::get_all_course(data).await;  
    let course_responses: Vec<Course> = courses.into_iter().collect::<Vec<Course>>();

    let json_response = serde_json::json!({
        "status" : "success",
        "result" : course_responses.len(),
        "courses:" : course_responses
    });
    HttpResponse::Ok().json(json_response)
}

///get request to fetch 1 course with a unique id
///return the course with a 200 resppcode
/// errors: sqlx row not found and basic not found err
/// resp with code 404
#[get("/courses/{id}")]
async fn get_course_by_id(data : web::Data<AppState>, path : web::Path<i32>) -> impl Responder{

    let course_id = path.into_inner();
    let query_res = service::service::get_course_by_id(data, course_id).await;

      match query_res{
        Ok(course) => {
            let course_resp = serde_json::json!({
                "status" : "success",
                "data" : serde_json::json!({
                    "course" : &course
                })
            });
            return HttpResponse::Ok().json(course_resp);

        },
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status" : "fail",
                "message" : format!("Cannot find the course with id: {}", course_id)
            }));
        },
        Err(err) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status" : "fail",
                "message" : format!("{:?}", err)
            }));

        }
        
    };

}

//---------------------------------------------COURSE RELATED-------------------------------------------------

//---------------------------------------------USER RELATED-------------------------------------------------

///Get a user datas measured by id
#[get("/user/{id}")]
async fn get_user_by_id(data : web::Data<AppState>, path : web::Path<i32>)->impl Responder{
    let u_id = path.into_inner();
    let query_result = service::service::get_user_by_maya_id(data, u_id).await;

    match query_result{
        Ok(user) => {
            let query_resp = serde_json::json!({
                "status" : "success",
                "data"   : serde_json::json!({
                "user"   : &user
                }),
            });
            return HttpResponse::Ok().json(query_resp);
        }
        Err(err) => {
            let query_resp = serde_json::json!({
                "status" : "fail",
                "message" : format!("There was an error with the request: {:?}", err)
            });

            return HttpResponse::BadRequest().json(query_resp);
        }
    }


}


///post request for creating user
///requirement: username, password and an email
/// namae,email,password
/// if user exists with same datas it returns Duplicate error
#[post("/user/")]
async fn create_user(body: web::Json<UserModel>, data : web::Data<AppState>)->impl Responder{
    //let u_id:i32 = 0;
    let query = service::service::create_user(body, data).await.map_err(|err: sqlx::Error| err.to_string());


    if let Err(err) = query{
        if err.contains("Duplicate entry"){
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status" : "fail",
                "message" : "Duplicate error!"
            }),
        );
        }
        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status" : "fail",
            "message" : format!("{:?}", err)
        }))
    }else{
        return HttpResponse::Ok().body("Successfully created the user!");
    }

   

}
//---------------------------------------------USER RELATED-------------------------------------------------


#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("heyho")
}
//service configurations, add routes and scope
pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/maya")
    .service(getCourses)
    .service(get_course_by_id)
    .service(create_user)
    .service(index);

    conf.service(scope);    
}
}
