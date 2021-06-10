#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

use crate::interface::handler;


pub mod domain;
pub mod infrastructure;
pub mod repository;
pub mod interface;
pub mod use_case;
pub mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new()
        .route("/task", web::get().to(handler::task::get_all))
        .route("/task/{id}", web::get().to(handler::task::get_by_id))
        .route("/task", web::post().to(handler::task::create_task))
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
