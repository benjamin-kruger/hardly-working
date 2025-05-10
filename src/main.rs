use clap::Parser;
use cli::Cli;
use colored::Colorize;
use hw::cli;
use hw::config;
use hw::task;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    let mut config = config::TodoListLocation::load();
    let mut task_list = task::TodoList::load(&config);

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
            task_list.add_task(description.clone(), group.clone());
        }
        Some(cli::Commands::List) => {
            task_list.list_tasks();
        }
        Some(cli::Commands::Toggle { task_id }) => match task_list.toggle_task(*task_id) {
            Ok(_) => {}
            Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
        },
        Some(cli::Commands::Remove { task_id }) => match task_list.remove_task(*task_id) {
            Ok(_) => {}
            Err(e) => eprintln!("{}: {}", "Error".red().bold(), e),
        },
        Some(cli::Commands::Search {
            partial_description,
        }) => {
            let results = task_list.search_tasks(partial_description);

            if results.is_empty() {
                println!(
                    "{}: No tasks found matching '{}'",
                    "Info".blue().bold(),
                    partial_description
                );
                return;
            }
            for (index, task) in results {
                task_list.display_task(index, task);
            }
        }
        Some(cli::Commands::Edit {
            task_id,
            description,
        }) => match task_list.edit_task(*task_id, description.clone()) {
            Ok(_) => {
                task_list.list_tasks();
            }
            Err(e) => eprintln!("{}: {}", "Error".to_string().red().bold(), e),
        },
        Some(cli::Commands::Clear {}) => {
            task_list.clear();
        }
        _ => {}
    }

    // Save tasks only if Add, Toggle, or Remove was executed
    if matches!(
        cli.command,
        Some(cli::Commands::Add { .. })
            | Some(cli::Commands::Toggle { .. })
            | Some(cli::Commands::Remove { .. })
            | Some(cli::Commands::Edit { .. })
            | Some(cli::Commands::Clear { .. })
    ) {
        if let Err(e) = task_list.save(&config) {
            eprintln!("{}: Failed to save tasks: {}", "Error".red().bold(), e);
        }
    }
}
