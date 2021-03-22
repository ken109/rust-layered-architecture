use crate::{domain, infrastructure::in_memory, repository};

pub trait ITaskUseCase {
    fn get_all(&mut self) -> &Vec<domain::Task>;

    fn get_by_id(&mut self, id: u32) -> Option<domain::Task>;

    fn create_task(&mut self, content: String) -> &domain::Task;
}

pub struct TaskUseCase {
    pub(crate) task_repository: Box<dyn repository::TaskRepository>
}

impl TaskUseCase {
    pub fn new() -> Box<dyn ITaskUseCase> {
        Box::new(TaskUseCase { task_repository: in_memory::task::TaskInMemory::new() })
    }
}

impl ITaskUseCase for TaskUseCase {
    fn get_all(&mut self) -> &Vec<domain::Task> {
        self.task_repository.get_all()
    }

    fn get_by_id(&mut self, id: u32) -> Option<domain::Task> {
        self.task_repository.get_by_id(id)
    }

    fn create_task(&mut self, content: String) -> &domain::Task {
        self.task_repository.create_task(content)
    }
}
