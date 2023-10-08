use clap::{Parser, Subcommand};
use std::path::PathBuf;
use storage::Task;

mod config;
mod storage;

const CONFIG_PATH: &str = "./config.json";

#[derive(Parser)]
#[command(
    name = "CLI Task List",
    version = "1.0",
    about = "CLI task tracker built with rust",
    author = "Tom"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds a task group (and task if specified)
    Add {
        /// Task Group
        task_group: String,

        /// Task
        task: Option<String>,
    },

    /// Displays (all) task groups and tasks
    Show {
        /// Task Group
        task_group: Option<String>,
    },

    /// Removes a task group (and task if specified)
    Remove {
        /// Task Group
        task_group: String,

        /// Task
        task: Option<String>,
    },

    /// Changes user directory, or prints current directory if no argument is given
    Dir {
        /// New user directory
        dir: Option<PathBuf>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    let mut task_manager =
        storage::read_task_manager(&config::read_config(CONFIG_PATH).user_dir.to_str().unwrap());

    match &cli.command {
        Some(Commands::Add { task_group, task }) => {
            let task_group = task_group;
            let task = task;
            let task_group_index = task_manager
                .task_groups
                .iter()
                .position(|x| x.name == task_group.to_string());
            if let Some(task_group_index) = task_group_index {
                if let Some(task) = task {
                    task_manager.task_groups[task_group_index].tasks.push(Task {
                        content: task.to_string(),
                    });
                    println!(
                        "Successfully added {} to {}",
                        task.to_string(),
                        task_group.to_string()
                    );
                }
            } else {
                task_manager.task_groups.push(storage::TaskGroup {
                    name: task_group.to_string(),
                    tasks: Vec::new(),
                });
                print!("Successfully created {}", task_group.to_string());
                if let Some(task) = task {
                    let last_index = task_manager.task_groups.len() - 1;
                    task_manager.task_groups[last_index].tasks.push(Task {
                        content: task.to_string(),
                    });
                    println!(" and added {} to it", task.to_string());
                } else {
                    println!();
                }
            }
            storage::write_task_manager(
                &config::read_config(CONFIG_PATH).user_dir.to_str().unwrap(),
                &task_manager,
            );
        }
        Some(Commands::Show { task_group }) => {
            if let Some(task_group) = task_group {
                let task_group_index = task_manager
                    .task_groups
                    .iter()
                    .position(|x| x.name == task_group.to_string());
                if let Some(task_group_index) = task_group_index {
                    println!(
                        "Task Group: {}",
                        task_manager.task_groups[task_group_index].name
                    );
                    for (i, task) in task_manager.task_groups[task_group_index]
                        .tasks
                        .iter()
                        .enumerate()
                    {
                        println!("{}: {}", i + 1, task.content);
                    }
                } else {
                    println!("Task group not found");
                }
            } else {
                for task_group in task_manager.task_groups.iter() {
                    println!("Task Group: {}", task_group.name);
                    for (i, task) in task_group.tasks.iter().enumerate() {
                        println!("{}: {}", i + 1, task.content);
                    }
                    println!("------------------------");
                }
            }
        }
        Some(Commands::Remove { task_group, task }) => {
            let task_group = task_group;
            let task = task;
            let task_group_index = task_manager
                .task_groups
                .iter()
                .position(|x| x.name == task_group.to_string());
            if let Some(task_group_index) = task_group_index {
                if let Some(task) = task {
                    let task_index = task_manager.task_groups[task_group_index]
                        .tasks
                        .iter()
                        .position(|x| x.content == task.to_string());
                    if let Some(task_index) = task_index {
                        task_manager.task_groups[task_group_index]
                            .tasks
                            .remove(task_index);
                        println!(
                            "Successfully removed {} from {}",
                            task.to_string(),
                            task_group.to_string(),
                        );
                    } else {
                        println!("Task not found");
                    }
                } else {
                    task_manager.task_groups.remove(task_group_index);
                    println!("Successfully removed {}", task_group.to_string());
                }
            } else {
                println!("Task group not found");
            }
            storage::write_task_manager(
                &config::read_config(CONFIG_PATH).user_dir.to_str().unwrap(),
                &task_manager,
            );
        }
        Some(Commands::Dir { dir }) => {
            if let Some(dir) = dir {
                config::write_config(
                    CONFIG_PATH,
                    &config::Config {
                        user_dir: dir.clone(),
                    },
                );
            } else {
                let config: config::Config = config::read_config(CONFIG_PATH);
                println!("User directory: {}", config.user_dir.display());
            }
        }
        None => {}
    }
}
