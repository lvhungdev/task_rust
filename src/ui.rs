use crate::task::Task;

pub struct UI;

impl UI {
    pub fn display_tasks(tasks: &[Task]) {
        println!("Id Description");
        println!("-- -----------");

        for (index, task) in tasks.iter().enumerate() {
            println!("{}  {}", index + 1, task.name);
        }
    }
}
