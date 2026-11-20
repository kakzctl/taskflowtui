# Project 01 — TaskFlowTUI

A practical, keyboard-driven task manager built with Rust and Ratatui.

**Repository:** https://github.com/kakzctl/taskflowtui

## Product goal

TaskFlow is intended for real everyday use. It should make capturing, organizing and reviewing tasks quick, work offline, and preserve user data reliably.

## Release 1 features

### Tasks
- [ ] Create, view, edit and delete tasks.
- [ ] Status: Todo, In Progress, Done.
- [ ] Priority: Low, Normal, High, Urgent.
- [ ] Optional due date and description.
- [ ] Created and updated timestamps.

### Projects and Inbox
- [ ] Create, rename and delete projects.
- [ ] Assign tasks to projects.
- [ ] Keep unassigned tasks in the Inbox.
- [ ] Never silently delete tasks when a project is deleted.

### Finding and reviewing
- [ ] Search titles and descriptions.
- [ ] Filter by status, priority and project.
- [ ] Sort by priority, due date and creation date.
- [ ] Dashboard counts.
- [ ] Upcoming and overdue tasks.

### Reliability
- [ ] Local SQLite database.
- [ ] Versioned schema.
- [ ] Input validation and useful error messages.
- [ ] Tests for domain logic and persistence.
- [ ] Terminal restoration on normal exit and errors.

## Later releases

**Release 2:** tags, recurring tasks, Today/Upcoming views, bulk actions, undo, import/export and backup/restore.

**Release 3:** subtasks, dependencies, activity history, configurable shortcuts, themes and optional integrations.

## Out of scope for Release 1

Cloud synchronization, accounts, collaboration, calendar synchronization, AI features, and web/mobile interfaces.

## Proposed keyboard map

| Key | Action |
|---|---|
| `q` | Quit |
| `Esc` | Close dialog or cancel current action |
| `j` / `k`, arrows | Move selection |
| `a` | Add task |
| `e` | Edit selected task |
| `d` | Delete selected task (confirmation required) |
| `Space` | Toggle completion where appropriate |
| `/` | Search |
| `?` | Show help |
| `Enter` | Open or confirm selection |

Shortcuts can evolve as the UI is tested.

## Implementation milestones

1. **Terminal foundation:** Ratatui/Crossterm setup, event loop, quit handling and best-effort cleanup are implemented. Local compilation and manual resize/cleanup verification remain.
2. **First dashboard:** header, task list, footer, selection and empty state.
3. **Domain model:** task, status, priority, project and unit tests.
4. **SQLite storage:** schema, versioning, repositories, transactions and persistence tests.
5. **Task workflows:** forms, CRUD, status, priority, dates and project assignment.
6. **Search and review:** filters, sorting, dashboard counts, upcoming and overdue tasks.
7. **Release readiness:** error handling, recovery checks, tests and user documentation.

## Architecture direction

Introduce modules gradually:

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

## Dependencies to evaluate

- `ratatui`: terminal UI rendering.
- `crossterm`: terminal events and backend.
- `rusqlite`: SQLite database access.
- `chrono` or `time`: date and timestamp handling, choose one based on implementation needs.
- Add other dependencies only when a concrete requirement justifies them.

## Definition of done

TaskFlow is ready for its first usable release when tasks and projects can be managed end-to-end, data persists across restarts, search and filters work, errors are handled clearly, tests pass and the terminal is restored correctly.
