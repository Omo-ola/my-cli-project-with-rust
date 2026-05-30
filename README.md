# CLI Task Manager

A command line task manager built with Rust.

## Commands
cargo run -- add "description"     # Add a task
cargo run -- list                   # List all tasks
cargo run -- complete <id>          # Mark as complete
cargo run -- delete <id>            # Delete a task
cargo run -- edit <id> "new desc"   # Edit description

## Built With
- Rust
- serde / serde_json
