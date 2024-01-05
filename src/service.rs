pub mod service{

    use crate::{
        CourseModel::{Course, CourseModelResponse},
        CourseSchema::{CreateCourseSchema, UpdateCourseSchema},
        UserModel::{UserModel},
        AppState,
    };
    use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
    use serde_json::json;
    use sqlx_mysql::MySqlQueryResult;


    pub async fn get_all_course(data : web::Data<AppState>) ->Vec<Course>{
        return sqlx::query_as!(Course, r#"SELECT * FROM course"#).fetch_all(&data.db).await.unwrap();
    }

    pub async fn get_course_by_id(data : web::Data<AppState>, course_id : i32)->Result<Course, sqlx::Error>{
        return sqlx::query_as!(Course, r#"SELECT * FROM course WHERE id=?"#,course_id).fetch_one(&data.db).await.unwrap();
    }

    pub async fn get_user_by_maya_id(data : web::Data<AppState>, user_id : i32)->Result<UserModel, sqlx::Error>{
        return sqlx::query_as!(UserModel, r#"SELECT * FROM user WHERE id=?"#, user_id).fetch_one(&data.db).await.unwrap();
    }
    

    pub async fn create_user(body : web::Json<UserModel>, data : web::Data<AppState>)->Result<MySqlQueryResult, sqlx::Error>{
        return sqlx::query(r#"INSERT INTO user (id, neptun_id, maya_id, namae, email, password, user_logname) VALUES(NULL, ?, ?, ?, ?, SHA(?), ?)"#)
        //.bind(u_id.clone())
        .bind(body.namae.to_string())
        .bind(body.email.to_string())
        .bind(body.password.to_string())
        .execute(&data.db)
        .await
        //.map_err(|err: sqlx::Error| err.to_string());
    }

}