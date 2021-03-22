pub mod handler {
    pub mod task {
        use actix_web::{HttpResponse, Responder, web};

        use crate::{domain, use_case};

        pub async fn get_all() -> impl Responder {
            println!("get_all");
            HttpResponse::Ok().json(use_case::TaskUseCase::new().get_all())
        }

        pub async fn get_by_id(web::Path(id): web::Path<u32>) -> impl Responder {
            println!("get_todo");
            let task: Option<domain::Task> = use_case::TaskUseCase::new().get_by_id(id);
            if task.is_some() {
                HttpResponse::Ok().json(task.unwrap())
            } else {
                HttpResponse::NotFound().finish()
            }
        }

        pub async fn create_task(task: web::Json<domain::Task>) -> impl Responder {
            println!("post_todo");
            use_case::TaskUseCase::new().create_task(task.content.clone());
            HttpResponse::Created().finish()
        }
    }
}
