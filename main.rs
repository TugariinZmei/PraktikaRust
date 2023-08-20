use std::io;

struct Task {
    id: u32,
    name: String,
}

struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    fn add_task(&mut self, name: String) {
        let id = self.tasks.len() as u32 + 1;
        self.tasks.push(Task { id, name });
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|task| task.id != id);
    }

    fn sort_tasks(&mut self) {
        self.tasks.sort_by(|a, b| a.name.cmp(&b.name));
    }

    fn print_tasks(&self) {
        for task in &self.tasks {
            println!("Task {}: {}", task.id, task.name);
        }
    }
}

fn main() {
    let mut task_manager = TaskManager::new();

    loop {
        println!("1. Добавить задачу");
        println!("2. Удалить задачу");
        println!("3. Отсортировать задачи");
        println!("4. Вывести задачи");
        println!("5. Выйти");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Не удалось прочитать строку");

        match choice.trim().parse().expect("Введите число") {
            1 => {
                println!("Введите название задачи:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Не удалось прочитать строку");
                task_manager.add_task(name.trim().to_owned());
            }
            2 => {
                println!("Введите ID задачи для удаления:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Не удалось прочитать строку");
                let id: u32 = id.trim().parse().expect("Введите число");
                task_manager.remove_task(id);
            }
            3 => {
                task_manager.sort_tasks();
            }
            4 => {
                task_manager.print_tasks();
            }
            5 => {
                break;
            }
            _ => {
                println!("Некорректный выбор, попробуйте ещё раз");
            }
        }
    }
}