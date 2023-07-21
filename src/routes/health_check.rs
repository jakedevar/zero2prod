use actix_web::HttpResponse;

// pub mod health_check;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

