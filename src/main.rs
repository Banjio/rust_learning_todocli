mod task;
use task::{Task, Tasks};

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
