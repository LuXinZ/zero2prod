#![allow(clippy::toplevel_ref_arg)]
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
use actix_web::dev::Server;
use sqlx::PgPool;
use std::net::TcpListener;
use crate::routes::{health_check, subscribe};
use actix_web::{web, App, HttpServer};

pub fn run(listener: TcpListener, de_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection = web::Data::new(de_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
