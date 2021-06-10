pub mod handler {
    pub mod task {
        use actix_web::{HttpResponse, Responder, web};

        use crate::{domain, use_case};

        pub async fn get_all() -> impl Responder {
            HttpResponse::Ok().json(use_case::TaskUseCase::new().get_all())
        }

        pub async fn get_by_id(web::Path(id): web::Path<i64>) -> impl Responder {
            let task: Option<domain::task::Task> = use_case::TaskUseCase::new().get_by_id(id);
            if task.is_some() {
                HttpResponse::Ok().json(task.unwrap())
            } else {
                HttpResponse::NotFound().finish()
            }
        }

        pub async fn create_task(task: web::Json<domain::task::NewTask>) -> impl Responder {
            use_case::TaskUseCase::new().create_task(task.content.clone());
            HttpResponse::Created().finish()
        }
    }
}
