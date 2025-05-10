# Hardly Working - Another todo list CLI

Working hard or hardly working - A todo list CLI

## Features

- Add, toggle, edit, and remove tasks
- Search and group tasks
- Markdown-based storage format
- Configurable todo list file location

## Installation

Build from source:

```bash
cargo install hardly-working
```

## Usage

```bash
# Configure task file location (by default at ~/.config/hw/config.json)
hw config --file-path ~/Documents/tasks.md

# Show current configuration
hw config --show

# Add a new task
hw add "I have plans to work hard"

# List all tasks
hw ls

# Toggle task completion status
hw toggle 1

# Remove a task
hw remove 1

# Clear all completed tasks
hw clear
```

## Storage

Tasks are stored in a simple Markdown file with checkbox syntax:

```markdown
- [ ] Incomplete task
- [x] Completed task
```

By default, tasks are stored in `~/todo.md`. To se a preferred path, use `hw config`.

## License

This project is licensed under an MIT license (http://opensource.org/licenses/MIT)
