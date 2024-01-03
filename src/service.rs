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

    pub async fn get_user_by_maya_id(data : web::Data<AppState>, user_id : i32)->UserModel::UserModel{
        return sqlx::query_as!(UserModel::UserModel, r#"SELECT * FROM user WHERE maya_id=?"#, user_id).fetch_one(&data.db).await.unwrap();
    }
    

    /*pub async fn create_user(body : web::Json<UserModel::UserModel>, data : web::Data<AppState>)->Result<{unknown},{unknown}>{
        return sqlx::query(r#"INSERT INTO user (id, neptun_id, maya_id, namae, email, password, user_logname) VALUES(NULL, ?, ?, ?, ?, SHA(?), ?)"#)
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
    }
*/
}