use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use kuchiki::NodeRef;
use kuchiki::parse_html;
use kuchiki::traits::*;
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

    // Parse HTML into a DOM
    let document = parse_html().one(html_content.clone());

    // Collect all <p> nodes first to safely modify DOM
    let p_nodes: Vec<NodeRef> = document
        .select("p")
        .unwrap()
        .map(|m| m.as_node().clone())
        .collect();

    for node in p_nodes {
        // Collect all text nodes inside this <p>
        let text_nodes: Vec<_> = node.descendants().text_nodes().collect();

        for text_node in text_nodes {
            let new_text = match transform_type.as_str() {
                "uppercase" => text_node.borrow().to_uppercase(),
                "lowercase" => text_node.borrow().to_lowercase(),
                _ => text_node.borrow().to_string(),
            };
            let new_node = NodeRef::new_text(new_text);
            text_node.as_node().insert_after(new_node);
            text_node.as_node().detach();
        }
    }

    // Serialize DOM back to HTML string and skip <html><head></head><body> wrapper
    let body_node = document.select_first("body").unwrap().as_node().clone();
    let mut result_html = Vec::new();
    for child in body_node.children() {
        child.serialize(&mut result_html).unwrap();
    }
    let result_str = String::from_utf8(result_html).unwrap();

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(result_str)
}

#[get("/health_check")]
pub async fn healthcheck() -> impl Responder {
    //return message Done fo succesful response
    HttpResponse::Ok().body("Done")
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    //run on port 8080
    HttpServer::new(|| App::new().service(healthcheck).service(transform_request))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    //static string for all tests
    static HTML: &str =
        "<html><body><p>Hello <b>World</b></p>Am here<p>Another <i>Line</i></p></body></html>";
    #[actix_web::test]
    async fn test_uppercase_transform_unit() {
        let app = test::init_service(App::new().service(transform_request)).await;

        let req = test::TestRequest::post()
            .uri("/transform_request") // match handler route
            .set_json(&TransformData {
                transform: "uppercase".into(),
                html: HTML.to_owned(),
            })
            .to_request();

        let resp_bytes = test::call_and_read_body(&app, req).await;
        let resp_str = String::from_utf8(resp_bytes.to_vec()).unwrap();

        println!("{}", resp_str);

        // Check for nest tags preservation
        assert!(resp_str.contains("HELLO <b>WORLD</b>"));
        assert!(resp_str.contains("ANOTHER <i>LINE</i>"));
        // Non-<p> text remains unchanged
        assert!(resp_str.contains("Am here"));
    }

    #[actix_web::test]
    async fn test_lowercase_transform_unit() {
        let app = test::init_service(App::new().service(transform_request)).await;

        let req = test::TestRequest::post()
            .uri("/transform_request")
            .set_json(&TransformData {
                transform: "lowercase".into(),
                html: HTML.to_owned(),
            })
            .to_request();

        let resp_bytes = test::call_and_read_body(&app, req).await;
        let resp_str = String::from_utf8(resp_bytes.to_vec()).unwrap();

        println!("{}", resp_str);

        assert!(resp_str.contains("hello <b>world</b>"));
        assert!(resp_str.contains("another <i>line</i>"));
        assert!(resp_str.contains("Am here"));
    }
}
