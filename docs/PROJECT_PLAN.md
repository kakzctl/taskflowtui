# TaskFlowTUI — Project Plan

## 1. Project vision

Build a reliable, keyboard-driven terminal task manager using Rust, Ratatui and Crossterm.

This repository is dedicated exclusively to TaskFlow. Every other application in the wider learning journey will have its own separate GitHub repository.

## 2. Learning outcomes

Through TaskFlow, practice:

- Rust ownership, borrowing, structs, enums and error handling.
- Ratatui layouts, widgets, styling and rendering.
- Crossterm keyboard events, raw mode and terminal restoration.
- Application state and event-loop design.
- Separating UI rendering from business logic.
- JSON serialization and local file persistence.
- Unit testing, integration testing and documentation.
- Focused Git commits and repository maintenance.

## 3. Scope and features

### Core features

- Display tasks in a selectable list or table.
- Add tasks with input validation.
- Edit and delete existing tasks.
- Mark tasks complete or active.
- Assign task priorities.
- Search and filter tasks.
- Persist tasks locally in JSON.
- Display status messages and keyboard help.

### Non-goals for the first version

- Cloud synchronization.
- Multi-user collaboration.
- Network services or accounts.
- A complex plugin system.

These can be reconsidered after the local application is stable.

## 4. Implementation milestones

### Milestone 1 — Terminal foundation

- [ ] Create the Rust binary crate.
- [ ] Add Ratatui and Crossterm.
- [ ] Initialize the terminal and enter raw mode.
- [ ] Create the draw/event loop.
- [ ] Handle quit and terminal resize events.
- [ ] Restore terminal state safely on exit and errors.

### Milestone 2 — First interface

- [ ] Create header, task area and footer.
- [ ] Render sample tasks.
- [ ] Add selected-row styling.
- [ ] Show keyboard shortcuts.
- [ ] Handle empty task lists.

### Milestone 3 — Task model and state

- [ ] Define Task and Priority types.
- [ ] Track selected task.
- [ ] Implement navigation.
- [ ] Implement task creation, editing and deletion in the domain layer.
- [ ] Implement completion state changes.
- [ ] Add unit tests for task operations.

### Milestone 4 — Interactive forms

- [ ] Add task form.
- [ ] Add edit form.
- [ ] Validate user input.
- [ ] Show clear validation and status messages.
- [ ] Confirm deletion before removing a task.

### Milestone 5 — Search and filters

- [ ] Search by task title.
- [ ] Filter active and completed tasks.
- [ ] Filter or sort by priority.
- [ ] Keep selection valid when filtered results change.

### Milestone 6 — Persistence

- [ ] Choose a suitable local data directory.
- [ ] Load tasks on startup.
- [ ] Save changes safely.
- [ ] Handle missing, malformed or inaccessible data.
- [ ] Test persistence across application restarts.

### Milestone 7 — Quality and release

- [ ] Add unit and integration tests.
- [ ] Run formatting, Clippy and tests.
- [ ] Test keyboard behavior and terminal restoration.
- [ ] Test empty states and invalid input.
- [ ] Document installation, usage and shortcuts.
- [ ] Tag or document the first usable release.

## 5. Suggested architecture

Introduce modules gradually as they become necessary:

```text
src/
├── main.rs
├── app.rs
├── event.rs
├── storage.rs
├── models/
│   ├── mod.rs
│   └── task.rs
└── ui/
    ├── mod.rs
    ├── layout.rs
    └── tasks.rs
```

Responsibilities:

- `main.rs`: startup, terminal lifecycle and application entry point.
- `app.rs`: application state and actions.
- `event.rs`: keyboard and terminal events.
- `models/task.rs`: task data and domain behavior.
- `storage.rs`: JSON loading and saving.
- `ui/`: rendering only; avoid embedding task business logic in widgets.

## 6. Dependencies

Start with only what the first milestones require:

- `ratatui`: terminal UI rendering.
- `crossterm`: terminal events and backend.
- `serde` with derive: serialization support.
- `serde_json`: JSON persistence.

Add an error-handling crate only when its benefits are clear. Avoid dependencies that do not serve a concrete requirement.

## 7. GitHub workflow

Repository: https://github.com/kakzctl/taskflowtui

All TaskFlow source code, tests, documentation and configuration belong here. Other projects must use their own repositories.

Use focused commits, for example:

- `docs: add TaskFlow implementation plan`
- `feat: initialize terminal event loop`
- `feat: render task list`
- `feat: add task creation`
- `test: cover task state transitions`

Before completing a milestone, run:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
```

Never commit credentials, tokens, personal data, local runtime data or the Cargo `target/` directory.

## 8. Definition of done

TaskFlow is complete when:

- All agreed core features work.
- The application restores the terminal on exit.
- Tasks persist across restarts.
- Invalid input and storage errors are handled.
- Tests pass and Clippy/format checks are clean.
- The README documents installation, usage and key bindings.
