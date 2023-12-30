use actix_cors::Cors;
//use actix_web::web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("heyho")
}



#[actix_web::main]
async fn main()->std::io::Result<()>{
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let db_url = std::env::var("DATABASE_URL").expect("Cannot find the database url!");
    println!("{}", db_url);

    HttpServer::new(|| {
        App::new()
        .route("/", web::get().to(index))
        .route("/user", web::post().to(index))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}
