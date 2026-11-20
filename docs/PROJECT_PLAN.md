# TaskFlowTUI — Full Project Plan

## 1. Vision

Build a portfolio of 12 practical Rust terminal applications with Ratatui. The series is both a structured learning path and a collection of useful, independently runnable tools.

The work progresses from basic terminal rendering and event handling to asynchronous tasks, operating-system integration, databases, networking and larger application architecture.

## 2. Learning outcomes

By the end of the series, practice and demonstrate:

- Rust ownership, borrowing, enums, traits, generics and error handling.
- Ratatui layouts, widgets, styling and rendering.
- Crossterm terminal events, raw mode and terminal restoration.
- Application state modelling and event-driven architecture.
- Filesystem operations, serialization and persistence.
- Async tasks and safe communication between background work and the UI.
- Process execution and integration with external tools.
- Unit testing, integration testing, profiling and documentation.
- Git branching, focused commits and repository maintenance.

## 3. Project sequence

### Phase A — Beginner

#### 01. TaskFlow — TUI task manager

**Purpose:** Build a keyboard-driven task manager and learn the core Ratatui application pattern.

Features:
- List tasks in a selectable table or list.
- Add, edit and delete tasks.
- Mark tasks complete or active.
- Assign priority and optional due date.
- Search and filter tasks.
- Persist tasks locally as JSON.
- Show helpful empty states, validation messages and status feedback.

Learning topics: terminal lifecycle, event loop, layouts, widgets, state, structs/enums, CRUD, serde and file I/O.

Acceptance criteria:
- [ ] App launches and exits without leaving the terminal in raw mode.
- [ ] User can navigate tasks using the keyboard.
- [ ] Create, edit, delete and completion actions work.
- [ ] Data survives application restarts.
- [ ] Invalid input and storage errors are handled.
- [ ] Core task logic has unit tests.
- [ ] README documents installation, usage and key bindings.

Suggested modules: `app`, `event`, `ui`, `models/task`, `storage`.

#### 02. TermNotes — Terminal notes

Features: create, edit, view, delete, search and tag notes; persist notes locally; provide a readable editor/viewer split.

Learning topics: text input, editor state, text wrapping, search, serialization and file storage.

#### 03. File Explorer

Features: browse directories, move between parent/child directories, inspect metadata, sort and filter entries, and show a preview for supported text files.

Learning topics: filesystem APIs, paths, permissions, error handling and navigation state.

#### 04. FocusTimer

Features: configurable focus and break sessions, start/pause/reset, session history and progress display.

Learning topics: time handling, tick events, state transitions and redraw efficiency.

### Phase B — Intermediate

#### 05. SysScope — System monitor

Features: CPU, memory, disk and uptime summaries; process list; periodic refresh; selectable views.

Learning topics: system information APIs, background refresh, channels, charts and resource management.

#### 06. Git Dashboard

Features: inspect repository status, branches, recent commits and changed files; provide safe, explicit actions for common Git operations.

Learning topics: subprocesses, command output, exit codes, asynchronous command execution and Git concepts.

#### 07. Download Manager

Features: add HTTP/HTTPS downloads, queue and pause/resume where supported, progress display, destination selection and history.

Learning topics: async Rust, HTTP clients, streaming, cancellation, concurrency and partial-file handling.

#### 08. Database Explorer

Features: open SQLite databases, list tables, inspect schema, run read queries and render paginated results.

Learning topics: SQL, database connections, parameterized queries, result modelling and safe query handling.

### Phase C — Advanced

#### 09. Container Manager

Features: list containers and images, inspect status, view logs and manage container lifecycle with confirmation for destructive actions.

Learning topics: Docker Engine/API integration, subprocesses or API clients, streaming logs and robust error reporting.

#### 10. NetScope — Network monitor

Features: show interfaces, addresses and traffic counters; refresh metrics; display connection information where supported.

Learning topics: platform APIs, network data, sampling intervals and portability boundaries.

#### 11. DevCenter — Developer command center

Features: project shortcuts, Git summaries, task list, system overview and configurable developer commands in one interface.

Learning topics: composing modules, configuration, plugin-like command definitions and application architecture.

#### 12. SmartFind — Fast file search

Features: search selected directories, fuzzy matching, filters, previews, ignore rules and keyboard-driven result navigation.

Learning topics: traversal, indexing, matching algorithms, concurrency, caching and benchmarking.

## 4. Standard project workflow

Each project follows the same lifecycle:

1. **Plan:** define user stories, scope, non-goals and acceptance criteria.
2. **Design:** sketch screens, keyboard map, state transitions and module boundaries.
3. **Scaffold:** create the Cargo package, dependencies and initial README.
4. **Foundation:** terminal lifecycle, event handling and render loop.
5. **Core functionality:** implement domain logic before connecting it to widgets.
6. **Persistence/integration:** add storage or external system integration.
7. **Polish:** empty states, validation, errors, help screen and accessibility.
8. **Verify:** format, lint, test and manually exercise the application.
9. **Document:** installation, usage, shortcuts, architecture and limitations.
10. **Publish:** commit the finished milestone to GitHub and update progress.

We will work incrementally. Do not generate all 12 applications in one batch; each should be implemented, run and reviewed before moving on.

## 5. GitHub repository strategy

Repository: https://github.com/kakzctl/taskflowtui

- Keep all project source code in this repository.
- Use one directory per application under `projects/`.
- Give each project its own Cargo manifest and README.
- Keep project-specific tests beside the project.
- Use focused commits, for example:
  - `docs: add TaskFlow project plan`
  - `feat(taskflow): add terminal event loop`
  - `feat(taskflow): render task list`
  - `test(taskflow): cover task state transitions`
- Push completed milestones to the default branch, or use feature branches when a change is large.
- Do not commit `target/`, secrets, local configuration or generated runtime data.
- Keep the root README roadmap and progress checklist current.

## 6. Quality gates

Before marking a milestone complete:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
```

Also perform a manual terminal test for keyboard navigation, resizing, empty data, invalid input, errors and clean exit. Add platform-specific checks when a feature depends on Linux or another operating system.

## 7. Initial repository structure

```text
taskflowtui/
├── README.md
├── docs/
│   ├── PROJECT_PLAN.md
│   └── projects/
│       └── 01-taskflow.md
├── projects/
│   └── 01-taskflow/
│       ├── Cargo.toml
│       ├── README.md
│       ├── src/
│       └── tests/
└── .github/
    └── workflows/
```

The project directories will be added as each project begins. This avoids empty placeholders and keeps every committed milestone meaningful.

## 8. Current status

- [x] Confirm GitHub repository and default branch.
- [x] Define the 12-project roadmap.
- [x] Define development workflow and quality gates.
- [ ] Begin Project 01 — TaskFlow.
- [ ] Complete and verify TaskFlow.
- [ ] Continue through Projects 02–12.
