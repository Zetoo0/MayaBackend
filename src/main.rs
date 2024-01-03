mod CourseModel;
mod CourseSchema;
mod controller;
mod service;
mod UserModel;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub struct AppState{
    db : MySqlPool,
}



#[actix_web::main]
async fn main()->std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }


    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("Cannot find the database url!");
    let pool = match MySqlPoolOptions::new().max_connections(10).connect(&db_url).await{
        Ok(pool)=>{
            println!("Successful connection!");
            pool
        }
        Err(err) =>{
            println!("There was an error to connect with the mysql database: {:?}", err);
            std::process::exit(1);
        }
    };
                        
    println!("Server started loool, {}", db_url);

     HttpServer::new(move ||{  
        App::new()
        .app_data(web::Data::new(AppState{db : pool.clone()}))
        .configure(controller::controller::config)
        .wrap(Logger::default())
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
