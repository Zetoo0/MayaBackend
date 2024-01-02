pub mod service{

    use crate::{
        CourseModel::{Course, CourseModelResponse},
        CourseSchema::{CreateCourseSchema, UpdateCourseSchema},
        UserModel,
        AppState,
    };
    use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
    use serde_json::json;

    pub async fn get_all_course(data : web::Data<AppState>) ->Vec<Course>{
        return sqlx::query_as!(Course, r#"SELECT * FROM course"#).fetch_all(&data.db).await.unwrap();
    }

    pub async fn get_course_by_id(data : web::Data<AppState>, course_id : i32)->Course{
        return sqlx::query_as!(Course, r#"SELECT * FROM course WHERE id=?"#,course_id).fetch_one(&data.db).await.unwrap();
    }

}