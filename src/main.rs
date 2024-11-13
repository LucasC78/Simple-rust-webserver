use actix_web::{get, App, HttpServer, Responder};
use actix_files::Files;

#[get("/")]
async fn index() -> impl Responder {
    "just drag your project into the public folder, and if you add a /public at the end of your url you should see your projects! GOOD BYE!!!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)              
            .service(Files::new("/public", "./public").show_files_listing()) 
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}