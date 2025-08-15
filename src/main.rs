use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    HttpServer::new(|| App::new())
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
