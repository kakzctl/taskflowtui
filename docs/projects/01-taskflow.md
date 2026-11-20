# Project 01 — TaskFlowTUI

A practical, keyboard-driven task manager built with Rust and Ratatui.

**Repository:** https://github.com/kakzctl/taskflowtui

## Product goal

TaskFlow is intended for real everyday use. It should make capturing, organizing and reviewing tasks quick, work offline, and preserve user data reliably.

## Release 1 features

### Tasks
- [x] Create, view, edit and delete tasks (in-memory implementation).
- [x] Status: Todo, In Progress, Done.
- [x] Priority: Low, Normal, High, Urgent.
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
- [x] Required-title validation and useful status messages.
- [ ] Persistent storage and recovery.
- [ ] Tests for domain logic and persistence.
- [ ] Terminal restoration on normal exit and errors.

## Later releases

**Release 2:** tags, recurring tasks, Today/Upcoming views, bulk actions, undo, import/export and backup/restore.

**Release 3:** subtasks, dependencies, activity history, configurable shortcuts, themes and optional integrations.

## Out of scope for Release 1

Cloud synchronization, accounts, collaboration, calendar synchronization, AI features, and web/mobile interfaces.

## Current keyboard map

| Key | Action |
|---|---|
| `q` / `Esc` / `Ctrl+C` | Quit |
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `g` | Select first task |
| `G` | Select last task |
| `s` | Cycle selected task status |
| `p` | Cycle selected task priority |
| `?` | Show or close keyboard help |

Task changes are currently in memory only and are lost when the application exits. More shortcuts will be added as task workflows are implemented.

## Current task CRUD controls

| Key | Action |
|---|---|
| `a` | Add task |
| `e` | Edit selected task |
| `d` | Request deletion; confirm with `y` or cancel with `n` |
| `Tab` | Switch between title and description |
| `Enter` | Move to description or save |
| `Ctrl+U` | Clear the active field |
| `Esc` | Cancel the form |

Task CRUD is currently in-memory only. SQLite persistence is not yet implemented.

## Implementation milestones

1. **Terminal foundation:** Ratatui/Crossterm setup, event loop, quit handling and best-effort cleanup are implemented. Local compilation and manual resize/cleanup verification remain.
2. **First dashboard:** header summary, selectable task list, details panel, shortcut footer and sample tasks are implemented. Empty-state polish and manual resize verification remain.
3. **Domain model:** Task, status and priority types, state cycling and transition unit tests are implemented. Project model and assignment remain.
4. **SQLite storage:** schema, versioning, repositories, transactions and persistence tests.
5. **Task workflows:** in-memory create/edit/delete forms, status and priority cycling are implemented. Due dates and project assignment remain.
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
