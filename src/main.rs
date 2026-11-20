mod project;
mod task;

use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
use project::Project;
use task::{Priority, Task, TaskStatus};

type AppTerminal = Terminal<CrosstermBackend<Stdout>>;

/// Owns the terminal session and restores the user's terminal when dropped.
struct TerminalGuard {
    terminal: AppTerminal,
}

impl TerminalGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;

        let mut stdout = io::stdout();
        if let Err(error) = execute!(stdout, EnterAlternateScreen) {
            let _ = disable_raw_mode();
            return Err(error);
        }

        let backend = CrosstermBackend::new(stdout);
        match Terminal::new(backend) {
            Ok(terminal) => Ok(Self { terminal }),
            Err(error) => {
                let _ = execute!(io::stdout(), LeaveAlternateScreen);
                let _ = disable_raw_mode();
                Err(error)
            }
        }
    }

    fn terminal(&mut self) -> &mut AppTerminal {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Best-effort cleanup: Drop cannot return an error.
        let _ = self.terminal.show_cursor();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}

#[derive(Debug, Clone)]
struct TaskForm {
    editing_id: Option<u32>,
    title: String,
    description: String,
    active_field: usize,
}

impl TaskForm {
    fn new() -> Self {
        Self {
            editing_id: None,
            title: String::new(),
            description: String::new(),
            active_field: 0,
        }
    }

    fn from_task(task: &Task) -> Self {
        Self {
            editing_id: Some(task.id),
            title: task.title.clone(),
            description: task.description.clone(),
            active_field: 0,
        }
    }

    fn active_value_mut(&mut self) -> &mut String {
        if self.active_field == 0 {
            &mut self.title
        } else {
            &mut self.description
        }
    }

    fn handle_key(&mut self, code: KeyCode) -> FormAction {
        match code {
            KeyCode::Esc => FormAction::Cancel,
            KeyCode::Tab | KeyCode::Down | KeyCode::Up => {
                self.active_field = 1 - self.active_field;
                FormAction::Continue
            }
            KeyCode::Enter => {
                if self.active_field == 0 {
                    self.active_field = 1;
                    FormAction::Continue
                } else {
                    FormAction::Save
                }
            }
            KeyCode::Backspace => {
                self.active_value_mut().pop();
                FormAction::Continue
            }
            KeyCode::Char(character) => {
                if !character.is_control() {
                    self.active_value_mut().push(character);
                }
                FormAction::Continue
            }
            _ => FormAction::Continue,
        }
    }
}

#[derive(Debug, Clone)]
struct ProjectForm {
    editing_id: Option<u32>,
    name: String,
}

impl ProjectForm {
    fn new() -> Self {
        Self { editing_id: None, name: String::new() }
    }

    fn from_project(project: &Project) -> Self {
        Self { editing_id: Some(project.id), name: project.name.clone() }
    }

    fn handle_key(&mut self, code: KeyCode) -> FormAction {
        match code {
            KeyCode::Esc => FormAction::Cancel,
            KeyCode::Enter => FormAction::Save,
            KeyCode::Backspace => { self.name.pop(); FormAction::Continue }
            KeyCode::Char(ch) if !ch.is_control() => { self.name.push(ch); FormAction::Continue }
            _ => FormAction::Continue,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Focus {
    Projects,
    Tasks,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FormAction {
    Continue,
    Save,
    Cancel,
}

struct App {
    tasks: Vec<Task>,
    projects: Vec<Project>,
    selected: usize,
    selected_project: usize,
    focus: Focus,
    should_quit: bool,
    show_help: bool,
    form: Option<TaskForm>,
    project_form: Option<ProjectForm>,
    confirm_delete: bool,
    confirm_delete_project: bool,
    status_message: String,
}

impl App {
    fn new() -> Self {
        Self {
            tasks: vec![
                Task::new(
                    1,
                    "Review Rust ownership",
                    "Revise borrowing, references, and ownership rules.",
                    Some("Learning".to_string()),
                    TaskStatus::InProgress,
                    Priority::High,
                ),
                Task::new(
                    2,
                    "Build the TaskFlow dashboard",
                    "Create the first usable task list and keyboard navigation.",
                    Some("TaskFlow".to_string()),
                    TaskStatus::InProgress,
                    Priority::Urgent,
                ),
                Task::new(
                    3,
                    "Plan the week's work",
                    "Choose the most important tasks for the week.",
                    None,
                    TaskStatus::Todo,
                    Priority::Normal,
                ),
                Task::new(
                    4,
                    "Read 20 pages",
                    "Continue the current book.",
                    Some("Personal".to_string()),
                    TaskStatus::Todo,
                    Priority::Low,
                ),
                Task::new(
                    5,
                    "Set up the development environment",
                    "Install and verify the tools needed for the project.",
                    Some("TaskFlow".to_string()),
                    TaskStatus::Done,
                    Priority::Normal,
                ),
            ],
            projects: vec![
                Project::new(1, "Learning"),
                Project::new(2, "TaskFlow"),
                Project::new(3, "Personal"),
            ],
            selected: 0,
            selected_project: 0,
            focus: Focus::Tasks,
            should_quit: false,
            show_help: false,
            form: None,
            project_form: None,
            confirm_delete: false,
            confirm_delete_project: false,
            status_message: "Dashboard loaded with sample tasks.".to_string(),
        }
    }

    fn visible_task_indices(&self) -> Vec<usize> {
        self.tasks.iter().enumerate()
            .filter_map(|(index, task)| {
                let matches = if self.selected_project == 0 {
                    task.project.is_none()
                } else {
                    self.projects.get(self.selected_project - 1)
                        .is_some_and(|project| task.project.as_deref() == Some(project.name.as_str()))
                };
                matches.then_some(index)
            }).collect()
    }

    fn selected_task_id(&self) -> Option<u32> {
        let visible = self.visible_task_indices();
        visible.get(self.selected).map(|&index| self.tasks[index].id)
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }

        if let Some(form) = self.form.as_mut() {
            if code == KeyCode::Char('u') && modifiers.contains(KeyModifiers::CONTROL) {
                form.active_value_mut().clear();
                return;
            }
            match form.handle_key(code) {
                FormAction::Continue => {}
                FormAction::Cancel => {
                    self.form = None;
                    self.status_message = "Task form cancelled.".to_string();
                }
                FormAction::Save => self.save_form(),
            }
            return;
        }

        if let Some(form) = self.project_form.as_mut() {
            if code == KeyCode::Char('u') && modifiers.contains(KeyModifiers::CONTROL) {
                form.name.clear();
                return;
            }
            match form.handle_key(code) {
                FormAction::Continue => {}
                FormAction::Cancel => {
                    self.project_form = None;
                    self.status_message = "Project form cancelled.".to_string();
                }
                FormAction::Save => self.save_project_form(),
            }
            return;
        }

        if self.confirm_delete_project {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => self.delete_selected_project(),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.confirm_delete_project = false;
                    self.status_message = "Project deletion cancelled.".to_string();
                }
                _ => {}
            }
            return;
        }

        if self.confirm_delete {
            match code {
                KeyCode::Char('y') | KeyCode::Char('Y') => self.delete_selected(),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.confirm_delete = false;
                    self.status_message = "Deletion cancelled.".to_string();
                }
                _ => {}
            }
            return;
        }

        if matches!(code, KeyCode::Char('q') | KeyCode::Esc) {
            self.should_quit = true;
            return;
        }

        if self.show_help {
            if code == KeyCode::Char('?') { self.show_help = false; }
            return;
        }

        if code == KeyCode::Tab {
            self.focus = match self.focus {
                Focus::Projects => Focus::Tasks,
                Focus::Tasks => Focus::Projects,
            };
            return;
        }

        if self.focus == Focus::Projects {
            match code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.selected_project = (self.selected_project + 1).min(self.projects.len());
                    self.selected = 0;
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.selected_project = self.selected_project.saturating_sub(1);
                    self.selected = 0;
                }
                KeyCode::Char('g') => { self.selected_project = 0; self.selected = 0; }
                KeyCode::Char('G') => { self.selected_project = self.projects.len(); self.selected = 0; }
                KeyCode::Char('a') => self.project_form = Some(ProjectForm::new()),
                KeyCode::Char('e') if self.selected_project > 0 => {
                    if let Some(project) = self.projects.get(self.selected_project - 1) {
                        self.project_form = Some(ProjectForm::from_project(project));
                    }
                }
                KeyCode::Char('d') if self.selected_project > 0 => self.confirm_delete_project = true,
                KeyCode::Char('?') => self.show_help = true,
                _ => {}
            }
            return;
        }

        match code {
            KeyCode::Char('j') | KeyCode::Down => self.move_selection_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection_up(),
            KeyCode::Char('g') => { self.selected = 0; self.status_message = "Moved to the first task.".to_string(); }
            KeyCode::Char('G') => {
                self.selected = self.visible_task_indices().len().saturating_sub(1);
                self.status_message = "Moved to the last task.".to_string();
            }
            KeyCode::Char('a') => self.start_create(),
            KeyCode::Char('e') => self.start_edit(),
            KeyCode::Char('d') => {
                if self.selected_task_id().is_some() { self.confirm_delete = true; }
            }
            KeyCode::Char('s') => self.cycle_selected_status(),
            KeyCode::Char('p') => self.cycle_selected_priority(),
            KeyCode::Char('m') => self.assign_selected_to_current_project(),
            KeyCode::Char('?') => self.show_help = true,
            _ => {}
        }
    }

    fn move_selection_down(&mut self) {
        let count = self.visible_task_indices().len();
        if count > 0 { self.selected = (self.selected + 1).min(count - 1); }
    }

    fn move_selection_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn save_project_form(&mut self) {
        let Some(form) = self.project_form.take() else { return; };
        let name = form.name.trim();
        if name.is_empty() {
            self.status_message = "A project name is required.".to_string();
            self.project_form = Some(form);
            return;
        }
        if self.projects.iter().any(|project| {
            Some(project.id) != form.editing_id && project.name.eq_ignore_ascii_case(name)
        }) {
            self.status_message = "A project with that name already exists.".to_string();
            self.project_form = Some(form);
            return;
        }
        if let Some(id) = form.editing_id {
            if let Some(project) = self.projects.iter_mut().find(|project| project.id == id) {
                let old_name = project.name.clone();
                project.name = name.to_string();
                for task in &mut self.tasks {
                    if task.project.as_deref() == Some(old_name.as_str()) {
                        task.project = Some(project.name.clone());
                    }
                }
                self.status_message = format!("Renamed project to {name}.");
            }
        } else {
            let id = self.projects.iter().map(|p| p.id).max().unwrap_or(0) + 1;
            self.projects.push(Project::new(id, name));
            self.selected_project = self.projects.len();
            self.selected = 0;
            self.status_message = format!("Created project {name}.");
        }
    }

    fn delete_selected_project(&mut self) {
        if self.selected_project == 0 { return; }
        if let Some(project) = self.projects.get(self.selected_project - 1).cloned() {
            for task in &mut self.tasks {
                if task.project.as_deref() == Some(project.name.as_str()) {
                    task.project = None;
                }
            }
            self.projects.remove(self.selected_project - 1);
            self.selected_project = 0;
            self.selected = 0;
            self.status_message = format!("Deleted project {}; its tasks were moved to Inbox.", project.name);
        }
        self.confirm_delete_project = false;
    }

    fn assign_selected_to_current_project(&mut self) {
        let visible = self.visible_task_indices();
        let Some(&index) = visible.get(self.selected) else { return; };
        let project_name = if self.selected_project == 0 {
            None
        } else {
            self.projects.get(self.selected_project - 1).map(|p| p.name.clone())
        };
        self.tasks[index].project = project_name.clone();
        self.status_message = match project_name {
            Some(name) => format!("Assigned task to {name}."),
            None => "Moved task to Inbox.".to_string(),
        };
        self.selected = 0;
    }

    fn start_create(&mut self) {
        self.form = Some(TaskForm::new());
    }

    fn start_edit(&mut self) {
        let visible = self.visible_task_indices();
        if let Some(&index) = visible.get(self.selected) {
            self.form = Some(TaskForm::from_task(&self.tasks[index]));
        }
    }

    fn save_form(&mut self) {
        let Some(form) = self.form.take() else {
            return;
        };

        let title = form.title.trim();
        if title.is_empty() {
            self.status_message = "A task title is required.".to_string();
            self.form = Some(form);
            return;
        }

        if let Some(id) = form.editing_id {
            if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
                task.title = title.to_string();
                task.description = form.description.trim().to_string();
                self.status_message = format!("Updated task #{id}.");
            }
        } else {
            let id = self.tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
            let project = if self.selected_project == 0 { None } else {
                self.projects.get(self.selected_project - 1).map(|p| p.name.clone())
            };
            self.tasks.push(Task::new(
                id,
                title,
                form.description.trim(),
                project,
                TaskStatus::Todo,
                Priority::Normal,
            ));
            self.selected = self.tasks.len() - 1;
            self.status_message = format!("Created task #{id}.");
        }
    }

    fn delete_selected(&mut self) {
        let visible = self.visible_task_indices();
        if let Some(&index) = visible.get(self.selected) {
            let removed = self.tasks.remove(index);
            self.selected = self.selected.min(self.visible_task_indices().len().saturating_sub(1));
            self.status_message = format!("Deleted task #{}.", removed.id);
        }
        self.confirm_delete = false;
    }

    fn cycle_selected_status(&mut self) {
        let visible = self.visible_task_indices();
        if let Some(&index) = visible.get(self.selected) {
            let task = &mut self.tasks[index];
            task.status = task.status.next();
            self.status_message = format!("Task status changed to {}.", task.status);
        }
    }

    fn cycle_selected_priority(&mut self) {
        let visible = self.visible_task_indices();
        if let Some(&index) = visible.get(self.selected) {
            let task = &mut self.tasks[index];
            task.priority = task.priority.next();
            self.status_message = format!("Task priority changed to {}.", task.priority);
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = TerminalGuard::new()?;
    let mut app = App::new();

    run(terminal.terminal(), &mut app)
}

fn run(terminal: &mut AppTerminal, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(|frame| render(frame, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Ignore key-repeat/release events; handle each press once.
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key.code, key.modifiers);
                }
            }
        }
    }

    Ok(())
}

fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height.min(area.height)),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Length(width.min(area.width)),
            Constraint::Percentage(50),
        ])
        .split(vertical[1])[1]
}

fn render(frame: &mut ratatui::Frame<'_>, app: &App) {
    let area = frame.area();

    if let Some(form) = &app.form {
        render_task_form(frame, area, form);
        return;
    }
    if let Some(form) = &app.project_form {
        render_project_form(frame, area, form);
        return;
    }

    if app.confirm_delete_project {
        let popup = centered_rect(area, 60, 8);
        frame.render_widget(Clear, popup);
        let name = app.projects.get(app.selected_project.saturating_sub(1))
            .map(|p| p.name.as_str()).unwrap_or("selected project");
        let message = Paragraph::new(vec![
            Line::from(format!("Delete project {name}?")),
            Line::from("Its tasks will be moved to Inbox, not deleted."),
            Line::from(""),
            Line::from("y Confirm    n / Esc Cancel"),
        ]).block(Block::default().title(" Confirm project deletion ").borders(Borders::ALL))
          .wrap(Wrap { trim: true });
        frame.render_widget(message, popup);
        return;
    }

    if app.confirm_delete {
        let popup = centered_rect(area, 52, 7);
        frame.render_widget(Clear, popup);
        let message = Paragraph::new(vec![
            Line::from("Delete the selected task?"),
            Line::from(""),
            Line::from(vec![
                Span::styled("y", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(" Confirm    "),
                Span::styled("n / Esc", Style::default().fg(Color::Cyan)),
                Span::raw(" Cancel"),
            ]),
        ])
        .block(Block::default().title(" Confirm deletion ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
        frame.render_widget(message, popup);
        return;
    }

    if app.show_help {
        let help = Paragraph::new(vec![
            Line::from(Span::styled(
                "Keyboard shortcuts",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("j / ↓       Move down"),
            Line::from("k / ↑       Move up"),
            Line::from("g           First task"),
            Line::from("G           Last task"),
            Line::from("Tab         Switch Projects / Tasks"),
            Line::from("Projects: a add, e rename, d delete"),
            Line::from("Tasks: a add, e edit, d delete"),
            Line::from("m           Move task to selected project / Inbox"),
            Line::from("s           Cycle task status"),
            Line::from("p           Cycle task priority"),
            Line::from("s           Cycle task status"),
            Line::from("p           Cycle task priority"),
            Line::from("?           Close this help"),
            Line::from("q / Esc     Quit TaskFlow"),
            Line::from(""),
            Line::from("Changes currently live in memory only."),
        ])
        .block(Block::default().title(" Help ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });

        frame.render_widget(help, area);
        return;
    }

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    let total = app.tasks.len();
    let done = app
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Done)
        .count();
    let active = total - done;

    let header = Paragraph::new(Line::from(vec![
        Span::styled(" TaskFlow ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(format!(
            "{active} active / {total} total  |  {}",
            app.status_message
        )),
    ]))
    .block(Block::default().borders(Borders::ALL));

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(22), Constraint::Percentage(43), Constraint::Percentage(35)])
        .split(sections[1]);

    let mut project_items = vec![ListItem::new("Inbox")];
    project_items.extend(app.projects.iter().map(|project| ListItem::new(project.name.as_str())));
    let project_list = List::new(project_items)
        .block(Block::default().title(" Projects ").borders(Borders::ALL))
        .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol("» ");
    let mut project_state = ListState::default();
    project_state.select(Some(app.selected_project));
    frame.render_stateful_widget(project_list, panels[0], &mut project_state);

    let visible_indices = app.visible_task_indices();
    let task_items: Vec<ListItem<'_>> = visible_indices.iter().map(|&index| &app.tasks[index])
        .map(|task| {
            let status_color = match task.status {
                TaskStatus::Todo => Color::Gray,
                TaskStatus::InProgress => Color::Yellow,
                TaskStatus::Done => Color::Green,
            };
            let priority_color = match task.priority {
                Priority::Low => Color::Gray,
                Priority::Normal => Color::White,
                Priority::High => Color::Yellow,
                Priority::Urgent => Color::Red,
            };

            ListItem::new(Line::from(vec![
                Span::styled(
                    format!("{} ", task.status.symbol()),
                    Style::default().fg(status_color),
                ),
                Span::raw(task.title.as_str()),
                Span::styled(
                    format!("  [{}]", task.priority),
                    Style::default().fg(priority_color),
                ),
            ]))
        })
        .collect();

    let selected_project_name = if app.selected_project == 0 { "Inbox" } else {
        app.projects.get(app.selected_project - 1).map(|p| p.name.as_str()).unwrap_or("Projects")
    };
    let tasks = List::new(task_items)
        .block(
            Block::default()
                .title(format!(" {selected_project_name} ({}) ", visible_indices.len()))
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("» ");

    let mut list_state = ListState::default();
    if !visible_indices.is_empty() {
        list_state.select(Some(app.selected));
    }
    frame.render_stateful_widget(tasks, panels[1], &mut list_state);

    let details = if let Some(&task_index) = visible_indices.get(app.selected) {
        let task = &app.tasks[task_index];
        vec![
            Line::from(Span::styled(
                task.title.as_str(),
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Gray)),
                Span::raw(task.status.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(Color::Gray)),
                Span::raw(task.priority.to_string()),
            ]),
            Line::from(vec![
                Span::styled("Project: ", Style::default().fg(Color::Gray)),
                Span::raw(task.project.as_deref().unwrap_or("Inbox")),
            ]),
            Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Gray)),
                Span::raw(task.id.to_string()),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "Description",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(task.description.as_str()),
        ]
    } else {
        vec![Line::from("No tasks yet. Press 'a' to add one.")]
    };

    let details = Paragraph::new(details)
        .block(Block::default().title(" Details ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(details, panels[2]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" j/k ", Style::default().fg(Color::Cyan)),
        Span::raw("Navigate  "),
        Span::styled("a ", Style::default().fg(Color::Cyan)),
        Span::raw("Add  "),
        Span::styled("e ", Style::default().fg(Color::Cyan)),
        Span::raw("Edit  "),
        Span::styled("d ", Style::default().fg(Color::Cyan)),
        Span::raw("Delete  "),
        Span::styled("m ", Style::default().fg(Color::Cyan)),
        Span::raw("Move  "),
        Span::styled("s ", Style::default().fg(Color::Cyan)),
        Span::raw("Status  "),
        Span::styled("p ", Style::default().fg(Color::Cyan)),
        Span::raw("Priority  "),
        Span::styled("? ", Style::default().fg(Color::Cyan)),
        Span::raw("Help  "),
        Span::styled("q ", Style::default().fg(Color::Cyan)),
        Span::raw("Quit"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, sections[0]);
    frame.render_widget(footer, sections[2]);
}

fn render_task_form(frame: &mut ratatui::Frame<'_>, area: Rect, form: &TaskForm) {
    let popup = centered_rect(area, 72, 15);
    frame.render_widget(Clear, popup);

    let title = if form.editing_id.is_some() {
        " Edit task "
    } else {
        " New task "
    };

    let title_style = if form.active_field == 0 {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    let description_style = if form.active_field == 1 {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let form_layout = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Length(5),
            Constraint::Length(1),
        ])
        .split(popup);

    let title_label = Paragraph::new("Title *").style(title_style);
    let title_input = Paragraph::new(form.title.as_str())
        .style(title_style)
        .block(Block::default().borders(Borders::ALL));
    let description_label = Paragraph::new("Description").style(description_style);
    let description_input = Paragraph::new(form.description.as_str())
        .style(description_style)
        .block(Block::default().borders(Borders::ALL));
    let hints = Paragraph::new("Tab: switch field  Enter: next/save  Ctrl+U: clear field  Esc: cancel");

    frame.render_widget(
        Block::default().title(title).borders(Borders::ALL),
        popup,
    );
    frame.render_widget(title_label, form_layout[0]);
    frame.render_widget(title_input, form_layout[1]);
    frame.render_widget(description_label, form_layout[2]);
    frame.render_widget(description_input, form_layout[3]);
    frame.render_widget(hints, form_layout[4]);
}



fn render_project_form(frame: &mut ratatui::Frame<'_>, area: Rect, form: &ProjectForm) {
    let popup = centered_rect(area, 58, 8);
    frame.render_widget(Clear, popup);
    let title = if form.editing_id.is_some() { " Rename project " } else { " New project " };
    let layout = Layout::default().direction(Direction::Vertical).margin(1)
        .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Length(1)]).split(popup);
    let input = Paragraph::new(form.name.as_str())
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().title("Name *").borders(Borders::ALL));
    frame.render_widget(Block::default().title(title).borders(Borders::ALL), popup);
    frame.render_widget(Paragraph::new("Enter: save  Ctrl+U: clear  Esc: cancel"), layout[0]);
    frame.render_widget(input, layout[1]);
    frame.render_widget(Paragraph::new("Project names must be unique."), layout[2]);
}

    #[test]
    fn project_creation_rejects_duplicate_names_case_insensitively() {
        let mut app = App::new();
        app.project_form = Some(ProjectForm { editing_id: None, name: "learning".into() });
        app.save_project_form();
        assert_eq!(app.projects.len(), 3);
        assert!(app.project_form.is_some());
        assert_eq!(app.status_message, "A project with that name already exists.");
    }

    #[test]
    fn renaming_project_updates_task_assignments() {
        let mut app = App::new();
        app.project_form = Some(ProjectForm { editing_id: Some(2), name: "Development".into() });
        app.save_project_form();
        assert!(app.tasks.iter().filter(|t| t.project.as_deref() == Some("Development")).count() > 0);
        assert!(!app.tasks.iter().any(|t| t.project.as_deref() == Some("TaskFlow")));
    }

    #[test]
    fn deleting_project_moves_its_tasks_to_inbox() {
        let mut app = App::new();
        app.selected_project = 2;
        app.confirm_delete_project = true;
        app.delete_selected_project();
        assert!(!app.projects.iter().any(|p| p.name == "TaskFlow"));
        assert!(app.tasks.iter().filter(|t| t.project.is_none()).count() >= 2);
        assert_eq!(app.selected_project, 0);
    }

    #[test]
    fn task_creation_uses_selected_project() {
        let mut app = App::new();
        app.selected_project = 1;
        let mut form = TaskForm::new();
        form.title = "Study ownership".into();
        app.form = Some(form);
        app.save_form();
        assert_eq!(app.tasks.last().unwrap().project.as_deref(), Some("Learning"));
    }

    #[test]
    fn tab_switches_between_project_and_task_navigation() {
        let mut app = App::new();
        app.handle_key(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(app.focus, Focus::Projects);
        app.handle_key(KeyCode::Tab, KeyModifiers::NONE);
        assert_eq!(app.focus, Focus::Tasks);
    }

#[cfg(test)]
mod tests {
    use super::{App, Focus, ProjectForm, TaskForm};
    use crossterm::event::{KeyCode, KeyModifiers};
    use crate::task::{Priority, TaskStatus};

    #[test]
    fn create_task_adds_it_to_the_list_and_selects_it() {
        let mut app = App::new();
        let original_count = app.tasks.len();

        let mut form = TaskForm::new();
        form.title = "Write integration tests".to_string();
        form.description = "Cover task workflows".to_string();
        app.form = Some(form);
        app.save_form();

        assert_eq!(app.tasks.len(), original_count + 1);
        assert_eq!(app.selected, original_count);
        assert_eq!(app.tasks[app.selected].title, "Write integration tests");
        assert_eq!(app.tasks[app.selected].description, "Cover task workflows");
        assert_eq!(app.tasks[app.selected].status, TaskStatus::Todo);
        assert_eq!(app.tasks[app.selected].priority, Priority::Normal);
    }

    #[test]
    fn empty_task_title_is_rejected_without_losing_the_form() {
        let mut app = App::new();
        let original_count = app.tasks.len();
        app.form = Some(TaskForm::new());

        app.save_form();

        assert_eq!(app.tasks.len(), original_count);
        assert!(app.form.is_some());
        assert_eq!(app.status_message, "A task title is required.");
    }

    #[test]
    fn editing_task_preserves_status_priority_and_project() {
        let mut app = App::new();
        let original = app.tasks[0].clone();

        let mut form = TaskForm::from_task(&original);
        form.title = "Updated title".to_string();
        form.description = "Updated description".to_string();
        app.form = Some(form);
        app.save_form();

        let updated = &app.tasks[0];
        assert_eq!(updated.title, "Updated title");
        assert_eq!(updated.description, "Updated description");
        assert_eq!(updated.status, original.status);
        assert_eq!(updated.priority, original.priority);
        assert_eq!(updated.project, original.project);
    }

    #[test]
    fn delete_requires_confirmation_and_keeps_selection_valid() {
        let mut app = App::new();
        let original_count = app.tasks.len();

        app.handle_key(KeyCode::Char('d'), KeyModifiers::NONE);
        assert!(app.confirm_delete);
        assert_eq!(app.tasks.len(), original_count);

        app.handle_key(KeyCode::Char('y'), KeyModifiers::NONE);
        assert!(!app.confirm_delete);
        assert_eq!(app.tasks.len(), original_count - 1);
        assert!(app.selected < app.tasks.len());
    }

    #[test]
    fn cancelling_delete_keeps_the_task() {
        let mut app = App::new();
        let original_count = app.tasks.len();

        app.handle_key(KeyCode::Char('d'), KeyModifiers::NONE);
        app.handle_key(KeyCode::Char('n'), KeyModifiers::NONE);

        assert_eq!(app.tasks.len(), original_count);
        assert!(!app.confirm_delete);
    }
}
