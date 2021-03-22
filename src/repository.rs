use crate::domain;

pub trait TaskRepository {
    fn get_all(&mut self) -> &Vec<domain::Task>;

    fn get_by_id(&mut self, id: u32) -> Option<domain::Task>;

    fn create_task(&mut self, content: String) -> &domain::Task;
}
