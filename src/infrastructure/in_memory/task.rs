use rand::Rng;

use crate::{domain, repository};

static mut TASK_LIST: TaskInMemoryInstance = TaskInMemoryInstance(Vec::new());

#[derive(Debug)]
struct TaskInMemoryInstance(Vec<domain::Task>);

impl TaskInMemoryInstance {
    fn get_all(&mut self) -> &Vec<domain::Task> {
        return &self.0;
    }

    fn get_by_id(&mut self, id: u32) -> Option<domain::Task> {
        let filtered: Vec<&domain::Task> = self.0.iter()
            .filter(|task| task.id.unwrap() == id)
            .collect::<Vec<&domain::Task>>();
        return if filtered.len() > 0 { Option::from(filtered[0].clone()) } else { Option::None };
    }

    fn create_task(&mut self, content: String) -> &domain::Task {
        self.0.push(domain::Task {
            id: Option::from(rand::thread_rng().gen::<u32>()),
            content,
            done: Option::from(false),
        });
        return &self.0[self.0.len() - 1];
    }
}

#[derive(Clone)]
pub struct TaskInMemory {}

impl TaskInMemory {
    pub fn new() -> Box<dyn repository::TaskRepository> {
        Box::new(TaskInMemory {})
    }
}

impl repository::TaskRepository for TaskInMemory {
    fn get_all(&mut self) -> &Vec<domain::Task> {
        unsafe {
            TASK_LIST.get_all()
        }
    }

    fn get_by_id(&mut self, id: u32) -> Option<domain::Task> {
        unsafe {
            TASK_LIST.get_by_id(id)
        }
    }

    fn create_task(&mut self, content: String) -> &domain::Task {
        unsafe {
            TASK_LIST.create_task(content)
        }
    }
}
