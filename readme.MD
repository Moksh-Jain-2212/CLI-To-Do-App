# Todo CLI

A simple interactive to-do list application written in Rust. Add tasks, view your list, mark tasks as completed, check their status, and delete them from the terminal.

## Requirements

- Rust with support for the 2024 edition
- Cargo

The project uses only Rust's standard library and has no external dependencies.

## Run the application

From this repository's root directory:

```bash
cd todo_cli
cargo run
```

To build an optimized executable:

```bash
cargo build --release
./target/release/todo_cli
```

On Windows, run `target\release\todo_cli.exe` instead.

## Usage

Enter a menu number and press Enter. Follow the prompts to provide a task name or task index.

| Option | Action |
| --- | --- |
| `0` | Add a new task |
| `1` | View all tasks |
| `2` | Mark a task as completed |
| `3` | Delete a task |
| `4` | Exit the application |
| `5` | Check the completion status of one task |

New tasks start as pending. Task status is printed as `true` for completed or `false` for pending. The task list uses Rust's debug format, for example:

```text
Task { name_of_task: "Learn Rust", task_id: 0, is_completed: false }
```

### Example workflow

1. Enter `0`, then type `Learn Rust` to add a task.
2. Enter `1` to view the task list.
3. Enter `2`, then `0` to complete the first task.
4. Enter `5`, then `0` to check its status; the application prints `true`.
5. Enter `3`, then `0` to delete the task.
6. Enter `4` to exit.

## Project structure

```text
.
├── readme.MD
└── todo_cli/
    ├── Cargo.toml     # Package metadata and dependencies
    ├── Cargo.lock     # Dependency lockfile
    └── src/
        └── main.rs   # Task model, menu, and task operations
```

Tasks are stored in a `Vec<Task>`. Each task contains a name, a numeric ID, and a completion flag. Input is read from standard input using `std::io`.

## Current limitations

- Tasks exist only in memory and are lost when the application exits.
- Task operations use the current zero-based list index. Deleting a task shifts later indices, but displayed `task_id` values are not updated. Use the task's current position in the list, starting at `0`.
- Non-numeric input causes the application to panic. Out-of-range task indices are retried without an explanatory message; task operations on an empty list have no valid index.
- The exit option currently calls `panic!`, so it prints an error message and exits with a failure status.
- Menu navigation uses recursive function calls, so very long sessions may exhaust the stack.
