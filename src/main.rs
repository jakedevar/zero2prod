use actix_web::{get, web, App,HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(req: HttpRequest) -> Responder {
    HttpResponse::Ok().finish()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
