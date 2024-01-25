use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

async fn health() -> HttpResponse {
    tracing::info!("Getting health");
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}
