
#[derive(Default)]
struct Task {
    description: String,
    is_done: Option<bool>
}

struct Tasks {
    tasks: Vec<Task>
}

impl Tasks {
    fn new() -> Self{
        Tasks { tasks: Vec::new() }
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.push(task);

    }

    fn get_task(&mut self, pos: usize) -> Option<&mut Task> {
        return self.tasks.get_mut(pos - 1)
    }
    
    fn list_tasks(&self){
        for (index, task) in self.tasks.iter().enumerate(){
            match task.is_done{
                Some(true) => println!("[x] Task {} is {}", index + 1, task.description),
                Some(false) | None => println!("[ ] Task {} is {}", index + 1, task.description)
            }
        }
    }
}

fn main() {
    let task1 = Task{
        description: String::from("Buy bannanas."),
        ..Default::default()
    };

    let task2: Task = Task{
        description: String::from("Buy chicken strips."),
        ..Default::default()
    };

   let mut tasks1 = Tasks::new();
   tasks1.add_task(task1);
   tasks1.add_task(task2);
   if let Some(task) = tasks1.get_task(1){
    task.is_done = Some(true)
   }
   tasks1.list_tasks();


}
