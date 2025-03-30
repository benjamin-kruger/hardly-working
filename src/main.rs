mod cli;
mod config;
mod task;

use clap::Parser;
use cli::Cli;
use colored::*;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    let mut config = config::TodoListLocation::load();
    let mut task_list = task::TaskList::load(&config);

    match &cli.command {
        Some(cli::Commands::Config { file_path, show }) => {
            if *show {
                println!("TODO List location: {}", config.file_path.display());
                return;
            }

            if let Some(path) = file_path {
                let path_buf = PathBuf::from(path);
                config.update_file_path(path_buf.clone());

                if let Err(e) = config.save() {
                    eprintln!("{}: Failed to save config: {}", "Error".red().bold(), e);
                    return;
                }
                println!(
                    "{} TODO List location set to: {}",
                    "Success:".green().bold(),
                    path_buf.display()
                );
            } else {
                println!(
                    "{} Use --file-path to set the TODO List location or --show to view current settings.",
                    "Info:".blue().bold()
                );
            }
        }
        Some(cli::Commands::Add { description, group }) => {
            let description = description.join(" ");
            task_list.add_task(description.clone(), group.clone());

            println!("{} {}", "Added task:".green(), description);
            if let Some(group_name) = group {
                println!("{} {}", "In group:".green(), group_name);
            }
            println!();
            task_list.list_tasks();
        }
        Some(cli::Commands::List) => {
            task_list.list_tasks();
        }
        Some(cli::Commands::Toggle { line }) => match task_list.toggle_task(*line) {
            Ok(_) => {
                let status = if task_list.tasks[*line - 1].completed {
                    "Completed".green()
                } else {
                    "Uncompleted".blue()
                };
                println!("{} task at line {}", status, line);
                println!();
                task_list.list_tasks();
            }
            Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
        },
        Some(cli::Commands::Remove { line }) => match task_list.remove_task(*line) {
            Ok(_) => {
                println!("{} task {}", "Removed".yellow(), line);
                println!();
                task_list.list_tasks();
            }
            Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
        },
        _ => {}
    }

    // Save tasks only if Add, Toggle, or Remove was executed
    if matches!(
        cli.command,
        Some(cli::Commands::Add { .. })
            | Some(cli::Commands::Toggle { .. })
            | Some(cli::Commands::Remove { .. })
    ) {
        if let Err(e) = task_list.save(&config) {
            eprintln!("{}: Failed to save tasks: {}", "Error".red().bold(), e);
        }
    }
}
