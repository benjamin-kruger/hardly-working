use crate::config::TodoListLocation;
use colored::*;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub struct Task {
    pub description: String,
    pub completed: bool,
    pub group: Option<String>,
}

pub struct TaskList {
    pub tasks: Vec<Task>,
}

impl TaskList {
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

    pub fn toggle_task(&mut self, line: usize) -> Result<(), String> {
        if line == 0 || line > self.tasks.len() {
            return Err(format!("Task {} does not exist", line));
        }
        let task = &mut self.tasks[line - 1];
        task.completed = !task.completed;
        Ok(())
    }

    pub fn remove_task(&mut self, line: usize) -> Result<(), String> {
        if line == 0 || line > self.tasks.len() {
            return Err(format!("Task {} does not exist", line));
        }
        self.tasks.remove(line - 1);
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

        let mut grouped: HashMap<Option<String>, Vec<(usize, &Task)>> = HashMap::new();
        for (i, task) in self.tasks.iter().enumerate() {
            grouped
                .entry(task.group.clone())
                .or_default()
                .push((i, task));
        }

        if let Some(tasks) = grouped.get(&None) {
            for &(i, task) in tasks {
                self.display_task(i, task);
            }
        }

        let mut groups: Vec<_> = grouped.iter().filter(|(k, _)| k.is_some()).collect();
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

    fn display_task(&self, index: usize, task: &Task) {
        let line_num = format!("{}:", index + 1);
        let checkbox = if task.completed {
            "[x]".green().bold()
        } else {
            "[ ]".red()
        };
        let description = if task.completed {
            task.description.green()
        } else {
            task.description.white()
        };
        println!("{} {} {}", line_num.blue().bold(), checkbox, description);
    }

    fn to_markdown(&self) -> String {
        let mut markdown = String::new();

        markdown.push_str("# Hardly Working TODO List\n\n");

        let mut grouped: HashMap<Option<String>, Vec<&Task>> = HashMap::new();
        for task in &self.tasks {
            grouped.entry(task.group.clone()).or_default().push(task);
        }

        if let Some(tasks) = grouped.get(&None) {
            for task in tasks {
                let checkbox = if task.completed { "[x]" } else { "[ ]" };
                markdown.push_str(&format!("- {} {}\n", checkbox, task.description));
            }
            markdown.push('\n');
        }

        let mut groups: Vec<_> = grouped.iter().filter(|(k, _)| k.is_some()).collect();
        groups.sort_by(|(a, _), (b, _)| a.cmp(b));

        for (group, tasks) in groups {
            if let Some(group_name) = group {
                markdown.push_str(&format!("## {}\n", group_name));
                for task in tasks {
                    let checkbox = if task.completed { "[x]" } else { "[ ]" };
                    markdown.push_str(&format!("- {} {}\n", checkbox, task.description));
                }
                markdown.push('\n');
            }
        }

        markdown
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

    pub fn load(config: &TodoListLocation) -> Self {
        let path = PathBuf::from(&config.file_path);
        let mut file = match File::open(&path) {
            Ok(file) => file,
            Err(_) => return TaskList::new(),
        };
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_err() {
            return TaskList::new();
        }
        TaskList::from_markdown(&contents)
    }

    pub fn save(&self, config: &TodoListLocation) -> io::Result<()> {
        let path = PathBuf::from(&config.file_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let markdown = self.to_markdown();
        let mut file = File::create(path)?;
        file.write_all(markdown.as_bytes())?;
        Ok(())
    }
}
