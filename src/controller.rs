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

#[get("/courses")]
pub async fn getCourses(data : web::Data<AppState>)->impl Responder{
   // let course_id = path.into_inner();

  /* match sqlx::query_as::<_,CourseModel>("SELECT course_id, namae FROM course").fetch_all(&data.db).await{
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(_) => HttpResponse::NotFound().json("Cannot find any course"),
   }*/

 //   let a: Vec<CourseModel> = sqlx::query_as!(CourseModel, "r# SELECT * FROM course").fetch_all(&data.db).await.unwrap();
    let courses: Vec<Course> = service::service::get_all_course(data).await;  //sqlx::query_as!(CourseModel, r#"SELECT * FROM course"#).fetch_all(&data.db).await.unwrap();
    let course_responses: Vec<Course> = courses.into_iter().collect::<Vec<Course>>();

    let json_response = serde_json::json!({
        "status" : "success",
        "result" : course_responses.len(),
        "courses:" : course_responses
    });
    HttpResponse::Ok().json(json_response)
}

#[get("/courses/{id}")]
async fn get_course_by_id(data : web::Data<AppState>, path : web::Path<i32>) -> impl Responder{

    let course_id = path.into_inner();
    let query_res: Course = service::service::get_course_by_id(data, course_id).await;

     let course_resp = serde_json::json!({
        "status" : "success",
        "data" : serde_json::json!({
            "course" : &query_res
        }),
    });
    return HttpResponse::Ok().json(course_resp);

    /*  match query_res{
        Ok(course) => {
            let course_resp = serde_json::json!({
                "status" : "success",
                "data" : serde_json::json!({
                    "course" : &course
                })
            });
            return HttpResponse::Ok().json(course_resp);

        },
        Err(err) => {
            return HttpResponse::NotFound().json(serde_json::json!({
                "status" : "fail",
                "message" : format!("{:?}", err)
            }));

        }
         Err(sqlx::Error::RowNotFound) => {
            return HttpResponse:NotFound().json(serde_json::json!({
                "status" : "fail",
                "message" : format!("Cannot find the course with id: {}", course_id)
            }));
        }
    };*/

}

//---------------------------------------------COURSE RELATED-------------------------------------------------

//---------------------------------------------USER RELATED-------------------------------------------------

#[post("/user/")]
async fn create_user(body: web::Json<UserModel>, data : web::Data<AppState>)->impl Responder{
    //let u_id:i32 = 0;
    let query = sqlx::query(r#"INSERT INTO user (id, neptun_id, maya_id, namae, email, password, user_logname) VALUES(NULL, ?, ?, ?, ?, SHA(?), ?)"#)
    //.bind(u_id.clone())
    .bind(body.neptun_id.to_string())
    .bind(body.maya_id.to_string())
    .bind(body.namae.to_string())
    .bind(body.email.to_string())
    .bind(body.password.to_string())
    .bind(body.user_logname.to_string())
    .execute(&data.db)
    .await
    .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query{
        if err.contains("Duplicate entry"){
            return HttpResponse::BadRequest().json(serde_json::json!({
                "status" : "fail",
                "message" : "duplicate lol"
            }),
        );
        }

        return HttpResponse::InternalServerError().json(serde_json::json!({
            "status" : "fail",
            "message" : format!("{:?}", err)
        }))
    }else{
        return HttpResponse::Ok().body("anyd");
    }

   

}
//---------------------------------------------USER RELATED-------------------------------------------------


#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("heyho")
}

pub fn config(conf: &mut web::ServiceConfig){
    let scope = web::scope("/api")
    .service(getCourses)
    .service(get_course_by_id)
    .service(create_user)
    .service(index);

    conf.service(scope);    
}
}
//#[get(course/id)]