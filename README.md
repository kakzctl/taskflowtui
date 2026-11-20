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

Press `q`, `Esc` or `Ctrl+C` to exit the initial screen.

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
- [ ] Initialize the Rust application.
- [ ] Implement terminal lifecycle and event loop.
- [ ] Build the first dashboard.
- [ ] Implement task and project operations.
- [ ] Add SQLite persistence.
- [ ] Add search, filters and sorting.
- [ ] Add tests, recovery and user documentation.

## License

License to be selected before the first public source-code release.
