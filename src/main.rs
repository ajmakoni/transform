use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize)]
struct TransformData {
    transform: String,
    html: String,
}


#[post("/transform_request")]
pub async fn transform_request(req: web::Json<TransformData>) -> impl Responder {
    // Extract tramsform data from Json 
    let transform_type = req.transform.to_lowercase();
    let html_content = &req.html;

   let result_str=format!("{} {}", transform_type, html_content);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(result_str)
}

#[get("/health_check")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Done")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    HttpServer::new(|| App::new().service(healthcheck).service(transform_request))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};

    #[actix_web::test]
    async fn verify_transportpost_unit() {
        let app = test::init_service(App::new().service(transform_request)).await;
let html:String ="<html><body><p>Hello <b>World</b></p>Am here<p>Another <i>Line</i></p></body></html>".into();
        let req = test::TestRequest::post()
            .uri("/transform_request") // matches the handler route
            .set_json(&TransformData {
                transform: "uppercase".into(),
                html:html.clone(),
            })
            .to_request();

        let resp_bytes = test::call_and_read_body(&app, req).await;
        let resp_str = String::from_utf8(resp_bytes.to_vec()).unwrap();

        println!("{}", resp_str);

        // Now the nested tags are preserved
        assert!(resp_str.contains(&html));
    }

}
