# TaskFlowTUI

A keyboard-driven terminal task manager built with Rust and Ratatui.

**Repository:** https://github.com/kakzctl/taskflowtui

## Scope

This repository is dedicated to **TaskFlow only**. It will contain TaskFlow's complete source code, tests, documentation, configuration and Git history.

Other applications in the wider Ratatui learning journey will each have their own separate GitHub repository. They will not be added to this repository.

## Goals

- Build a practical, keyboard-driven terminal task manager.
- Learn idiomatic, modular and maintainable Rust.
- Use Ratatui for rendering and Crossterm for terminal interaction.
- Separate application state, event handling, UI rendering, task models and persistence.
- Add tests, error handling and documentation as the application grows.

## Planned features

- Display tasks in a selectable list or table.
- Add, edit and delete tasks.
- Mark tasks complete or active.
- Assign priorities and optional due dates.
- Search and filter tasks.
- Save and load tasks locally as JSON.
- Provide keyboard shortcuts and helpful status messages.

See [the detailed project plan](docs/PROJECT_PLAN.md) and [implementation milestones](docs/projects/01-taskflow.md).

## Development workflow

We will build the application in small, reviewable milestones:

1. Define requirements and acceptance criteria.
2. Design the application flow and module structure.
3. Set up the Rust project and dependencies.
4. Implement one feature at a time.
5. Run formatting, tests and relevant checks.
6. Document usage, shortcuts and design decisions.
7. Commit completed milestones to this repository.

### Clone

```bash
git clone https://github.com/kakzctl/taskflowtui.git
cd taskflowtui
```

## Development standards

- Use stable Rust unless there is a documented reason not to.
- Prefer small modules with clear responsibilities.
- Restore the terminal safely on normal exit and errors.
- Keep UI rendering separate from task/business logic.
- Avoid unnecessary dependencies; explain why each dependency is used.
- Add unit tests for task logic and integration tests where appropriate.
- Run the following before completing a milestone:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build
```

- Do not commit `target/`, credentials, API tokens, personal data or local runtime files.

## Progress

- [x] Define TaskFlow scope and features.
- [x] Document implementation milestones.
- [ ] Initialize the Rust application.
- [ ] Implement the terminal foundation.
- [ ] Build the task management interface.
- [ ] Implement task operations and persistence.
- [ ] Add tests and complete documentation.

## License

License to be selected before the first public source-code release.
