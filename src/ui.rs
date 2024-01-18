use crate::task::Task;

pub struct UI;

impl UI {
    pub fn display_tasks(tasks: &[Task]) {
        let tasks_len: usize = tasks.len().to_string().len();
        let tasks_len = if tasks_len < 2 { 2 } else { tasks_len };
        let space: String = (0..tasks_len - 2).map(|_| " ").collect();

        println!("Id {} Description", space);
        println!("-- {} -----------", space);

        for (index, task) in tasks.iter().enumerate() {
            let id_len: usize = index.to_string().len();
            let space_to_fill: usize = tasks_len - id_len;

            let inner_space: String = (0..space_to_fill).map(|_| " ").collect();

            println!("{}{}  {}", index + 1, inner_space, task.name);
        }
    }
}
