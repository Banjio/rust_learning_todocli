
#[derive(Default)]
pub struct Task {
    pub description: String,
    pub is_done: Option<bool>
}

pub struct Tasks {
    pub tasks: Vec<Task>
}


pub struct TasksGroup {
    pub tasks: Vec<Task>,
    pub name: String
}


impl Tasks {
    pub fn new() -> Self{
        Tasks { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: Task) {
        self.tasks.push(task);

    }

    pub fn get_task(&mut self, pos: usize) -> Option<&mut Task> {
        return self.tasks.get_mut(pos - 1)
    }
    
    pub fn list_tasks(&self){
        for (index, task) in self.tasks.iter().enumerate(){
            match task.is_done{
                Some(true) => println!("[x] Task {} is {}", index + 1, task.description),
                Some(false) | None => println!("[ ] Task {} is {}", index + 1, task.description)
            }
        }
    }
}
