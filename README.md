# Hardly Working - Another todo list CLI

Working hard or hardly working - A todo list CLI

## Features

- Add, toggle, and remove tasks
- Colorful terminal output
- Markdown-based storage format
- Configurable todo list file location

## Installation

Build from source:

```bash
git clone https://github.com/benjamin-kruger/hardly-working
cd hardly-working
cargo build --release
cp target/release/hw ~/.cargo/bin/hw
```

## Usage

```bash
# Configure task file location
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
```

## Storage

Tasks are stored in a simple Markdown file with checkbox syntax:

```markdown
- [ ] Incomplete task
- [x] Completed task
```

By default, tasks are stored in `~/todo.md`, but you can change this with the `config` command.

## License

This project is licensed under an MIT license (http://opensource.org/licenses/MIT)
