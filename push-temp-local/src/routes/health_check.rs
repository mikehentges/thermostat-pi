use actix_web::HttpResponse;

pub(crate) async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
