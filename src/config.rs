use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoListLocation {
    pub file_path: PathBuf,
}

impl Default for TodoListLocation {
    fn default() -> Self {
        let default_path = dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("todo.md");

        Self {
            file_path: default_path,
        }
    }
}

impl TodoListLocation {
    pub fn load() -> Self {
        let config_path = get_config_file_path();

        let config_file = match File::open(&config_path) {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Warning: Config file not found. Using default configuration.");
                return TodoListLocation::default();
            }
        };

        let mut reader = io::BufReader::new(config_file);
        let mut contents = String::new();

        if reader.read_to_string(&mut contents).is_err() {
            eprintln!("Error: Failed to read config file. Using default configuration.");
            return TodoListLocation::default();
        }

        match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Error: Failed to parse config file. Using default configuration.");
                TodoListLocation::default()
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = get_config_file_path();

        if let Some(parent_path) = config_path.parent() {
            fs::create_dir_all(parent_path)?;
        }

        let file = File::create(config_path)?;
        serde_json::to_writer_pretty(file, self)?;

        Ok(())
    }

    pub fn update_file_path(&mut self, path: PathBuf) {
        self.file_path = path;
    }
}

fn get_config_file_path() -> PathBuf {
    let config_dir = dirs::config_dir()
        .or_else(|| dirs::home_dir())
        .unwrap_or_else(|| PathBuf::from("."));

    config_dir.join("hw").join("config.json")
}
