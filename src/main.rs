use actix_web::{App, HttpResponse, HttpServer, Responder, get, post};

#[get("/health_check")]
pub async fn healthcheck()->impl Responder{
    HttpResponse::Ok().body("Done")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    HttpServer::new(|| App::new().service(healthcheck))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn health_check(){
      let app = test::init_service(App::new().service(healthcheck)).await;

        // Use GET since the endpoint is a GET
        let req = test::TestRequest::get()
            .uri("/health_check")
            .to_request();

        let resp_bytes = test::call_and_read_body(&app, req).await;
        let resp_str = String::from_utf8(resp_bytes.to_vec()).unwrap();

        assert!(resp_str.contains("Done"));
    }
}