use crate::{domain, infrastructure::persistence, repository};

pub trait ITaskUseCase {
    fn get_all(&mut self) -> Vec<domain::task::Task>;

    fn get_by_id(&mut self, id: i64) -> Option<domain::task::Task>;

    fn create_task(&mut self, content: String) -> domain::task::NewTask;
}

pub struct TaskUseCase {
    pub(crate) task_repository: Box<dyn repository::TaskRepository>
}

impl TaskUseCase {
    pub fn new() -> Box<dyn ITaskUseCase> {
        Box::new(TaskUseCase { task_repository: persistence::task::TaskPersistence::new() })
    }
}

impl ITaskUseCase for TaskUseCase {
    fn get_all(&mut self) -> Vec<domain::task::Task> {
        self.task_repository.get_all()
    }

    fn get_by_id(&mut self, id: i64) -> Option<domain::task::Task> {
        self.task_repository.get_by_id(id)
    }

    fn create_task(&mut self, content: String) -> domain::task::NewTask {
        self.task_repository.create_task(content)
    }
}
