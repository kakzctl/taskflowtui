# TaskFlowTUI

A hands-on Rust and Ratatui learning series: build 12 practical terminal user interface (TUI) applications, from beginner to advanced.

**Repository:** https://github.com/kakzctl/taskflowtui

## Goals

- Learn Ratatui by building complete, useful applications.
- Practice idiomatic, modular and maintainable Rust.
- Separate application state, event handling, rendering, domain models and persistence.
- Add tests, error handling and documentation as each project grows.
- Keep the source code, exercises and progress in this repository.

## Project roadmap

| # | Project | Level | Main topics |
|---:|---|---|---|
| 01 | [TaskFlow](docs/projects/01-taskflow.md) | Beginner | Ratatui basics, CRUD, keyboard events, JSON persistence |
| 02 | TermNotes | Beginner | Text input, editing, search, file storage |
| 03 | File Explorer | Beginner | Filesystem navigation, metadata, sorting |
| 04 | FocusTimer | Beginner | Time, application state, progress indicators |
| 05 | SysScope | Intermediate | System metrics, background refresh, charts |
| 06 | Git Dashboard | Intermediate | Git integration, subprocesses, repository status |
| 07 | Download Manager | Intermediate | Async Rust, networking, queues, progress |
| 08 | Database Explorer | Intermediate | SQLite, SQL queries, tabular results |
| 09 | Container Manager | Advanced | Docker integration, container lifecycle, logs |
| 10 | NetScope | Advanced | Network interfaces, traffic metrics, connections |
| 11 | DevCenter | Advanced | Multi-tool integration, application architecture |
| 12 | SmartFind | Advanced | File indexing, fuzzy matching, previews |

## Repository layout

The repository will use a workspace-style layout as the series grows:

```text
taskflowtui/
├── README.md
├── Cargo.toml                 # Workspace manifest (introduced when useful)
├── docs/
│   ├── PROJECT_PLAN.md
│   └── projects/
│       └── 01-taskflow.md
├── projects/
│   ├── 01-taskflow/
│   ├── 02-termnotes/
│   ├── 03-file-explorer/
│   └── ...
└── .github/
    └── workflows/
```

Each project will be independently runnable and will have its own README, source code and tests. Shared code will only be extracted when there is a clear benefit; projects should remain understandable on their own.

## How we will work

Every project is developed in small, reviewable milestones:

1. Define requirements and acceptance criteria.
2. Design the application flow and module structure.
3. Set up the Rust project and dependencies.
4. Implement one feature at a time.
5. Run formatting, tests and relevant checks.
6. Document usage, shortcuts and design decisions.
7. Commit completed milestones to GitHub.

### Local workflow

```bash
git clone https://github.com/kakzctl/taskflowtui.git
cd taskflowtui
git pull
```

We will keep GitHub as the source of truth. Each completed milestone will be committed and pushed to this repository. Never commit credentials, API tokens, personal data or local build output.

## Development standards

- Use stable Rust unless a project has a documented reason not to.
- Prefer small modules with clear responsibilities.
- Handle terminal setup and teardown safely, including errors and panics where practical.
- Keep UI rendering separate from business logic.
- Avoid unnecessary dependencies; explain why each dependency is used.
- Add unit tests for domain logic and integration tests where appropriate.
- Run `cargo fmt --check`, `cargo clippy -- -D warnings` and `cargo test` before milestone completion.
- Document Linux-specific behavior and portability limitations.

## Progress

See [the full project plan](docs/PROJECT_PLAN.md) and [TaskFlow's first-project plan](docs/projects/01-taskflow.md).

- [ ] Project 01 — TaskFlow
- [ ] Project 02 — TermNotes
- [ ] Project 03 — File Explorer
- [ ] Project 04 — FocusTimer
- [ ] Project 05 — SysScope
- [ ] Project 06 — Git Dashboard
- [ ] Project 07 — Download Manager
- [ ] Project 08 — Database Explorer
- [ ] Project 09 — Container Manager
- [ ] Project 10 — NetScope
- [ ] Project 11 — DevCenter
- [ ] Project 12 — SmartFind

## License

License to be selected before the first public source-code release.
