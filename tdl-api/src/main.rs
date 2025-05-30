mod controllers;
mod services;
mod models;
mod db;
mod routes;
use sqlx::migrate::Migrator;
use actix_web::{web::Data, App, HttpServer};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::init().await;

    MIGRATOR.run(&db_pool).await.expect("Failed to run migrations");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .configure(routes::init)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}