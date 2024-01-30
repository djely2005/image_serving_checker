use actix_web::{get, web, App, HttpServer, HttpRequest, HttpResponse, Responder, Error};
#[actix_web::main]
async fn main() {
    let _ = HttpServer::new(|| 
        App::new()
        .route("/{uuid}.png", web::get().to(get_image))
    )
        .bind("127.0.0.1:8080")
        .expect("error binding server to address")
        .run()
        .await;
}

async fn get_image(req: HttpRequest) -> Result<HttpResponse, Error> {
    match std::fs::read("public/Spy.png") {
        Ok(image_content) => {
            Ok(HttpResponse::Ok()
                .content_type("image/png")
                .body(image_content))
        },
        Err(e) => {
            println!("get_image({}): error, {:?}", req.match_info().query("id").parse::<String>().unwrap(), e);
            Err(actix_web::Error::from(e))
        }
    }
}