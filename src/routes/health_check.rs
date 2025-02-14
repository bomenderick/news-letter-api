use actix_web::HttpResponse;

// We can also use "impl Responder" as return type for our routes.
// There's no performance different, just a coding style preference.
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
