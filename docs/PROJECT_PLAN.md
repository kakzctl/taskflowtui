# TaskFlowTUI — Product and Engineering Plan

## 1. Product vision

TaskFlowTUI is a local-first, keyboard-driven task manager for everyday use, built with Rust and Ratatui.

This is a practical application, not merely a tutorial demo. It should be dependable, easy to operate, protective of user data and maintainable as features grow.

This repository contains TaskFlow only. Every other application in the wider Ratatui learning journey will have its own separate GitHub repository.

## 2. Target use

A single user should be able to:

- Capture a task quickly without navigating complex menus.
- Organize tasks into projects or leave them in an Inbox.
- Set priorities and due dates.
- See what is due, overdue and completed.
- Search, filter and sort tasks.
- Close and reopen the application without losing data.

The initial target platform is Linux. Keep platform-specific behavior isolated where practical.

## 3. Release scope

### Release 1 — Core task manager

**Task management**
- Create, view, edit and delete tasks.
- Mark tasks Todo, In Progress or Done.
- Set Low, Normal, High or Urgent priority.
- Add an optional due date and description.
- Track created and updated timestamps.

**Organization**
- Create, rename and delete projects.
- Assign a task to a project or leave it in the Inbox.
- Prevent accidental loss of tasks when a project is deleted; ask how to handle its tasks or move them to Inbox.

**Finding and reviewing**
- Search task titles and descriptions.
- Filter by status, priority and project.
- Sort by priority, due date and creation date.
- Dashboard counts for total, active and completed tasks.
- Show upcoming and overdue tasks.

**Usability and reliability**
- Keyboard-first navigation and discoverable shortcuts.
- Clear validation, confirmation and status messages.
- Useful empty states.
- Local SQLite persistence.
- Graceful handling of database errors and interrupted operations.
- Tests for domain logic and persistence.

### Release 2 — Productivity

- Tags and categories.
- Today and Upcoming views.
- Recurring tasks.
- Bulk operations.
- Undo for supported accidental actions.
- JSON/CSV export and import.
- Backup and restore.
- Configurable data directory and preferences.

### Release 3 — Advanced

- Subtasks and task dependencies.
- Task history and activity tracking.
- Configurable keyboard shortcuts.
- Themes and date/time formats.
- Optional external integrations, only when there is a clear use case.

## 4. Non-goals for Release 1

- Accounts, authentication or multi-user collaboration.
- Cloud synchronization or hosted backend.
- Calendar synchronization.
- AI features.
- Web or mobile interface.
- Complex plugin architecture.

These are deliberately excluded so the first release can be finished, tested and trusted.

## 5. Data model

Use SQLite as the source of truth.

### Project
- Integer primary key.
- Name (required, unique).
- Optional description.
- Created and updated timestamps.

### Task
- Integer primary key.
- Title (required).
- Optional description.
- Status: Todo, In Progress or Done.
- Priority: Low, Normal, High or Urgent.
- Optional due date.
- Optional project foreign key; NULL means Inbox.
- Created and updated timestamps.

### Tag and TaskTag
The schema can accommodate tags later, but tag management is not required for Release 1. Avoid implementing unused features prematurely.

Use foreign keys and database transactions. Project deletion must never silently delete a user's tasks.

## 6. Engineering requirements

- Stable Rust.
- Ratatui for rendering and Crossterm for terminal interaction.
- SQLite for durable local storage.
- Keep UI rendering, application state, domain logic and storage separate.
- Validate input before writing to storage.
- Use parameterized SQL queries.
- Handle database failures visibly; never pretend a write succeeded.
- Keep schema changes versioned through migrations or a controlled schema-version mechanism.
- Test task transitions, project assignment, filtering and persistence.
- Avoid unnecessary dependencies.

## 7. Implementation milestones

### Milestone 1 — Terminal foundation
- [x] Create the Rust binary crate.
- [x] Add Ratatui and Crossterm.
- [x] Initialize raw mode and alternate screen.
- [x] Implement the draw/event loop.
- [x] Handle quit input.
- [x] Add best-effort terminal restoration on errors and normal exit.
- [ ] Verify compilation and manually test terminal resize/cleanup locally.

### Milestone 2 — First dashboard
- [x] Header with application name and task summary.
- [x] Main task list with selection and details panel.
- [x] Footer with keyboard shortcuts.
- [x] Sample in-memory tasks initially.
- [ ] Empty-state polish and manual resize verification.

### Milestone 3 — Domain model
- [x] Define Task, TaskStatus and Priority.
- [ ] Define Project.
- [x] Implement task status and priority cycling.
- [x] Add unit tests for status and priority transitions.
- [ ] Implement project assignment and Inbox behavior.

### Milestone 4 — SQLite storage
- [ ] Add SQLite connection and schema initialization.
- [ ] Add schema versioning.
- [ ] Implement task and project repositories.
- [ ] Use parameterized queries and transactions.
- [ ] Test create/read/update/delete and restart persistence.
- [ ] Choose a platform-appropriate application data directory.

### Milestone 5 — Interactive task management
- [x] Create and edit forms for task title and description.
- [x] Delete confirmation.
- [x] Completion/status and priority changes.
- [ ] Due-date editing.
- [ ] Project creation and assignment.
- [x] Required-title validation and status feedback.
- [x] Unit tests for create, edit, validation and delete workflows.

### Milestone 6 — Search and dashboard
- [ ] Search title and description.
- [ ] Status, priority and project filters.
- [ ] Sorting.
- [ ] Dashboard counts.
- [ ] Upcoming and overdue views.

### Milestone 7 — Release readiness
- [ ] Test empty, large and malformed-data scenarios.
- [ ] Verify no operation silently loses user data.
- [ ] Test terminal restoration on exit and errors.
- [ ] Run formatting, Clippy and all tests.
- [ ] Document installation, usage, shortcuts and data location.
- [ ] Prepare backup guidance and a first usable release.

## 8. Suggested architecture

Introduce modules as they are needed; do not create empty files in advance.

```text
src/
├── main.rs
├── app.rs
├── event.rs
├── models/
│   ├── mod.rs
│   ├── task.rs
│   └── project.rs
├── storage/
│   ├── mod.rs
│   ├── database.rs
│   ├── tasks.rs
│   └── projects.rs
└── ui/
    ├── mod.rs
    ├── layout.rs
    ├── dashboard.rs
    └── tasks.rs
```

## 9. GitHub and quality workflow

Repository: https://github.com/kakzctl/taskflowtui

All TaskFlow source, tests and documentation belong here. Other projects use separate repositories.

Use focused commits, for example:

- `feat: initialize terminal event loop`
- `feat: render task dashboard`
- `feat: add task domain model`
- `feat: add SQLite persistence`
- `test: cover task repository operations`

Before completing milestones:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
```

Never commit the local database, user data, secrets or Cargo `target/` output.

## 10. Definition of done

Release 1 is usable when the user can manage tasks and projects end-to-end, data persists across restarts, search and filters work, errors are handled honestly, tests pass, and the terminal is restored correctly after exit.
