# Project 01 — TaskFlow

A keyboard-driven terminal task manager built with Rust and Ratatui.

## Goal

Use TaskFlow to learn the reusable foundation for the remaining TUI projects: terminal lifecycle, event handling, application state, rendering, domain models, persistence and testing.

## Planned features

- [ ] Dashboard with task list and status summary.
- [ ] Keyboard navigation and selection.
- [ ] Add tasks with input validation.
- [ ] Edit and delete tasks.
- [ ] Toggle completion.
- [ ] Set priority.
- [ ] Search and filter.
- [ ] Save and load JSON data.
- [ ] Help panel showing shortcuts.
- [ ] Unit tests and usage documentation.

## Proposed keyboard map

| Key | Action |
|---|---|
| `q` / `Esc` | Quit or close the current dialog |
| `j` / `k` or arrows | Move selection |
| `a` | Add task |
| `e` | Edit selected task |
| `d` | Delete selected task (with confirmation) |
| `Space` | Toggle completion |
| `/` | Search |
| `?` | Show help |

Shortcuts may be adjusted as the interface is implemented.

## Implementation milestones

### 1. Terminal foundation
- [ ] Initialize Crossterm and Ratatui.
- [ ] Enter raw mode and alternate screen.
- [ ] Build a draw/event loop.
- [ ] Restore terminal on normal exit and errors.

### 2. First screen
- [ ] Create header, main task area and footer.
- [ ] Render sample tasks.
- [ ] Add selected-row styling.
- [ ] Handle terminal resizing.

### 3. Task model and state
- [ ] Define Task, Priority and completion state.
- [ ] Add selection and navigation.
- [ ] Implement and test task operations.

### 4. Interactive forms
- [ ] Add task form.
- [ ] Edit form.
- [ ] Validation and status messages.
- [ ] Delete confirmation.

### 5. Search and filters
- [ ] Search by task title.
- [ ] Filter active/completed tasks.
- [ ] Filter or sort by priority.

### 6. Persistence
- [ ] Define storage location and data format.
- [ ] Load tasks on startup.
- [ ] Save changes safely.
- [ ] Handle missing, invalid or inaccessible data.

### 7. Quality and release
- [ ] Add unit and integration tests.
- [ ] Run formatting, Clippy and tests.
- [ ] Test keyboard behavior and terminal restoration.
- [ ] Document installation, usage and shortcuts.
- [ ] Mark TaskFlow complete in the root README.

## Proposed module structure

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

We will introduce modules gradually rather than creating empty files upfront.

## Dependencies to evaluate

- `ratatui`: terminal UI rendering.
- `crossterm`: terminal events and backend.
- `serde` with derive: data serialization.
- `serde_json`: JSON persistence.
- `thiserror` or `anyhow`: error handling, selected when implementation needs it.

Avoid adding dependencies until their purpose is clear.

## Definition of done

TaskFlow is complete when all planned core features work, data persists across restarts, errors are handled, tests pass, the terminal is restored on exit, and the README explains how to build and use it.
