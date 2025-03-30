use crate::config::TodoListLocation;
use colored::Colorize;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub struct Task {
    pub description: String,
    pub completed: bool,
    pub group: Option<String>,
}

pub struct TodoList {
    pub tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, description: String, group: Option<String>) {
        self.tasks.push(Task {
            description,
            completed: false,
            group,
        });
    }

    pub fn toggle_task(&mut self, task_id: usize) -> Result<(), String> {
        if task_id == 0 || task_id > self.tasks.len() {
            return Err(format!("Task {} does not exist", task_id));
        }
        let task = &mut self.tasks[task_id - 1];
        task.completed = !task.completed;
        Ok(())
    }

    pub fn remove_task(&mut self, task_id: usize) -> Result<(), String> {
        if task_id == 0 || task_id > self.tasks.len() {
            return Err(format!("Task {} does not exist", task_id));
        }
        self.tasks.remove(task_id - 1);
        Ok(())
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!(
                "{}",
                "You're Hardly Working! Work harder with 'hw add <task>'".yellow()
            );
            return;
        }

        println!("{}", "TODO List:".bold().underline());

        let mut tasks_by_group: HashMap<Option<String>, Vec<(usize, &Task)>> = HashMap::new();
        for (task_id, task) in self.tasks.iter().enumerate() {
            tasks_by_group
                .entry(task.group.clone())
                .or_default()
                .push((task_id, task));
        }

        if let Some(tasks) = tasks_by_group.get(&None) {
            for &(i, task) in tasks {
                self.display_task(i, task);
            }
        }

        let mut groups: Vec<_> = tasks_by_group.iter().filter(|(k, _)| k.is_some()).collect();
        groups.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (group, tasks) in groups {
            if let Some(group_name) = group {
                println!("\n{}", group_name.blue().bold().underline());
                for &(i, task) in tasks {
                    self.display_task(i, task);
                }
            }
        }
    }

    pub fn edit_task(&mut self, task_id: usize, new_description: String) -> Result<(), String> {
        if task_id == 0 || task_id > self.tasks.len() {
            return Err(format!("Task {} does not exist", task_id));
        }

        let task = &mut self.tasks[task_id - 1];
        task.description = new_description;

        Ok(())
    }

    pub fn search_tasks(&self, partial_description: &str) -> Vec<(usize, &Task)> {
        self.tasks
            .iter()
            .enumerate()
            .filter(|(_, task)| {
                task.description
                    .to_lowercase()
                    .contains(&partial_description.to_lowercase())
            })
            .collect()
    }

    pub fn display_task(&self, index: usize, task: &Task) {
        let line_num = format!("{}:", index + 1);
        let checkbox = if task.completed {
            "[x]".green().bold()
        } else {
            "[ ]".red()
        };
        let description = if task.completed {
            task.description.green().strikethrough()
        } else {
            task.description.white()
        };
        println!("{} {} {}", line_num.blue().bold(), checkbox, description);
    }

    fn to_markdown(&self) -> String {
        let mut content = String::new();

        content.push_str("# Hardly Working TODO List\n\n");

        let mut tasks_by_group: HashMap<Option<String>, Vec<&Task>> = HashMap::new();
        for task in &self.tasks {
            tasks_by_group
                .entry(task.group.clone())
                .or_default()
                .push(task);
        }

        if let Some(tasks) = tasks_by_group.get(&None) {
            for task in tasks {
                let checkbox = if task.completed { "[x]" } else { "[ ]" };
                content.push_str(&format!("- {} {}\n", checkbox, task.description));
            }
            content.push('\n');
        }

        let mut groups: Vec<_> = tasks_by_group.iter().filter(|(k, _)| k.is_some()).collect();
        groups.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (group, tasks) in groups {
            if let Some(group_name) = group {
                content.push_str(&format!("## {}\n\n", group_name));
                for task in tasks {
                    let checkbox = if task.completed { "[x]" } else { "[ ]" };
                    content.push_str(&format!("- {} {}\n", checkbox, task.description));
                }
                content.push('\n');
            }
        }
        content
    }

    fn from_markdown(content: &str) -> Self {
        let mut tasks = Vec::new();
        let mut current_group: Option<String> = None;

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.starts_with("# ") {
                continue;
            }

            if line.starts_with("## ") {
                current_group = Some(line[3..].trim().to_string());
                continue;
            }

            if line.starts_with("- [") && line.len() > 6 {
                let completed = &line[3..4] == "x";
                let description = line[6..].trim().to_string();
                tasks.push(Task {
                    description,
                    completed,
                    group: current_group.clone(),
                });
            }
        }

        Self { tasks }
    }

    pub fn load(todo_list_location: &TodoListLocation) -> Self {
        let path = PathBuf::from(&todo_list_location.file_path);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return TodoList::new(),
        };
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return TodoList::new();
        }
        TodoList::from_markdown(&contents)
    }

    pub fn save(&self, todo_list_location: &TodoListLocation) -> io::Result<()> {
        let path = PathBuf::from(&todo_list_location.file_path);
        if let Some(parent_path) = path.parent() {
            fs::create_dir_all(parent_path)?;
        }
        let content = self.to_markdown();
        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
