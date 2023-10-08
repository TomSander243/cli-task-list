use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct TaskGroup {
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskManager {
    pub task_groups: Vec<TaskGroup>,
}

pub fn read_task_manager(path: &str) -> TaskManager {
    if Path::new(path).exists() {
        let mut file = File::open(path).expect("Unable to open task manager file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read task manager file");
        serde_json::from_str(&contents).expect("Unable to parse task manager file")
    } else {
        let task_manager = TaskManager {
            task_groups: Vec::new(),
        };
        write_task_manager(path, &task_manager);
        task_manager
    }
}

pub fn write_task_manager(path: &str, task_manager: &TaskManager) {
    let contents =
        serde_json::to_string_pretty(task_manager).expect("Unable to serialize task manager");
    fs::write(path, contents).expect("Unable to write task manager file");
}
