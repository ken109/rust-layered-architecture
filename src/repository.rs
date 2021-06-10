use crate::domain;

pub trait TaskRepository {
    fn get_all(&mut self) -> Vec<domain::task::Task>;

    fn get_by_id(&mut self, id: i64) -> Option<domain::task::Task>;

    fn create_task(&mut self, content: String) -> domain::task::NewTask;
}
