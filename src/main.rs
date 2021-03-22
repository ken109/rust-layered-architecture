use actix_web::{App, HttpServer, web};

use crate::interface::handler;

mod domain;
mod infrastructure;
mod repository;
mod interface;
mod use_case;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .route("/task", web::get().to(handler::task::get_all))
        .route("/task/{id}", web::get().to(handler::task::get_by_id))
        .route("/task", web::post().to(handler::task::create_task))
    )
    .bind("127.0.0.1:8080")?
        .run()
        .await
}
