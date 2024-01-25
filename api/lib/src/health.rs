use actix_web::{get, web};
use sqlx::PgPool;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}
#[get("/version")]
async fn version(db: web::Data<PgPool>) -> String {
    tracing::info!("Getting version");
    let result = sqlx::query_scalar("SELECT version()")
        .fetch_one(db.get_ref())
        .await;
    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {e:?}"),
    }
}
