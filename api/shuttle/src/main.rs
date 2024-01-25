use actix_files::NamedFile;
use actix_web::{
    get,
    web::{self, ServiceConfig},
    Responder,
};
use api_lib::film_repository::PostgresFilmRepository;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = PostgresFilmRepository::new(pool);
    let film_repository = actix_web::web::Data::new(film_repository);
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::films::service::<api_lib::film_repository::PostgresFilmRepository>,
                ),
        )
        .service(index);
    };

    Ok(config.into())
}
#[get("/")]
async fn index() -> impl Responder {
    NamedFile::open_async("static/index.html").await
}
