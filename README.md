# TaskFlowTUI

A practical, keyboard-driven task manager built with Rust and Ratatui.

**Repository:** https://github.com/kakzctl/taskflowtui

## Scope

This repository is dedicated to **TaskFlow only**. Every other application in the wider Ratatui learning journey will have its own separate GitHub repository.

TaskFlow is intended to become a tool used for real everyday work, not just a tutorial exercise. Reliability, data safety, usability and maintainability are core requirements.

## Product vision

A local-first terminal application for capturing, organizing and tracking everyday tasks. It should be fast to operate from the keyboard, useful without an internet connection, and dependable enough to use as a daily task manager.

## Release 1 — Core task manager

- Create, view, edit and delete tasks.
- Mark tasks as Todo, In Progress or Done.
- Assign Low, Normal, High or Urgent priority.
- Set optional due dates.
- Organize tasks into projects, with an Inbox for unassigned tasks.
- Create, rename and delete projects; deleting a project safely moves its tasks to Inbox.
- Search tasks and filter by status, priority and project.
- Sort by priority, due date or creation date.
- Show dashboard counts and upcoming/overdue tasks.
- Persist data locally in SQLite.
- Provide keyboard help, input validation and clear status/error messages.

## Later releases

**Release 2:** tags, recurring tasks, Today/Upcoming views, bulk actions, import/export, backup/restore and undo.

**Release 3:** subtasks, dependencies, task history, configurable shortcuts, themes and optional integrations.

## Explicit non-goals for the initial release

- User accounts or multi-user collaboration.
- Cloud synchronization or a hosted service.
- Calendar synchronization.
- AI features.
- A web or mobile interface.

## Build and run

Requirements: a recent stable Rust toolchain and Cargo.

```bash
cargo run
```

TaskFlow stores tasks and projects in a local SQLite database. Data is loaded on startup and saved after changes.

### Dashboard controls

| Key | Action |
| --- | --- |
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` | Select the first task |
| `G` | Select the last task |
| `s` | Cycle the selected task's status |
| `p` | Cycle the selected task's priority |
| `a` | Add a task |
| `e` | Edit selected task |
| `d` | Request deletion (confirm with `y`, cancel with `n`) |
| `Tab` | Switch focus between Projects and Tasks |
| `m` | Cycle selected task through projects and Inbox |
| `?` | Open keyboard help |
| `q` / `Esc` / `Ctrl+C` | Quit |

Task CRUD and project organization are implemented. Select Inbox or a project in the Projects panel to filter the task list. Use `Tab` to switch panels; in the Projects panel, `a` adds, `e` renames and `d` deletes a project. Deleting a project moves its tasks to Inbox. In the Tasks panel, `m` cycles a task through the available projects and Inbox. Changes are saved to SQLite automatically. On Linux, the database is stored in the application data directory (normally `~/.local/share/taskflowtui/taskflow.sqlite3`; `$XDG_DATA_HOME` is respected).

## Development approach

We will build this in small, testable milestones. Each milestone should leave the application in a working state.

1. Establish the Rust project and terminal lifecycle.
2. Build the first usable dashboard.
3. Implement task and project domain logic.
4. Add SQLite persistence.
5. Add search, filters and sorting.
6. Add validation, recovery and tests.
7. Polish documentation and prepare a usable release.

## Clone

```bash
git clone https://github.com/kakzctl/taskflowtui.git
cd taskflowtui
```

## Quality standards

- Use stable Rust and keep modules focused.
- Separate UI, application state, domain logic and persistence.
- Never silently discard user data.
- Handle database and terminal errors clearly.
- Add tests for core behavior and persistence.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test` and `cargo build` before completing milestones.
- Never commit credentials, personal data, runtime database files or `target/`.

## Progress

- [x] Define the product scope and release direction.
- [x] Initialize the Rust application.
- [x] Implement terminal lifecycle and event loop.
- [x] Build the dashboard with Vim-style navigation.
- [x] Implement task creation, editing and project operations.
- [x] Add SQLite persistence.
- [ ] Add search, filters and sorting.
- [ ] Complete recovery testing and release documentation.

## License

License to be selected before the first public source-code release.
