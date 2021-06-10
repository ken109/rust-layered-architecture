use crate::{repository, domain, schema};
use super::super::persistence::connect;
use diesel::{RunQueryDsl, QueryDsl};

#[derive(Clone)]
pub struct TaskPersistence {}

impl TaskPersistence {
    pub fn new() -> Box<dyn repository::TaskRepository> {
        Box::new(TaskPersistence {})
    }
}

impl repository::TaskRepository for TaskPersistence {
    fn get_all(&mut self) -> Vec<domain::task::Task> {
        schema::tasks::dsl::tasks
            .load::<domain::task::Task>(&connect())
            .expect("Error loading posts")
    }

    fn get_by_id(&mut self, id: i64) -> Option<domain::task::Task> {
        schema::tasks::dsl::tasks
            .find(id)
            .first::<domain::task::Task>(&connect())
            .ok()
    }

    fn create_task(&mut self, content: String) -> domain::task::NewTask {
        let task = domain::task::NewTask {
            content,
        };

        diesel::insert_into(schema::tasks::table)
            .values(&task)
            .execute(&connect())
            .expect("Error saving new post");

        task
    }
}
