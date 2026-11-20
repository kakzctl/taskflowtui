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
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Terminal,
};
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

struct App {
    tasks: Vec<Task>,
    selected: usize,
    should_quit: bool,
    show_help: bool,
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
            selected: 0,
            should_quit: false,
            show_help: false,
            status_message: "Dashboard loaded with sample tasks.".to_string(),
        }
    }

    fn handle_key(&mut self, code: KeyCode, modifiers: KeyModifiers) {
        if code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }

        if matches!(code, KeyCode::Char('q') | KeyCode::Esc) {
            self.should_quit = true;
            return;
        }

        if self.show_help {
            if matches!(code, KeyCode::Char('?') | KeyCode::Esc) {
                self.show_help = false;
            }
            return;
        }

        match code {
            KeyCode::Char('j') | KeyCode::Down => self.move_selection_down(),
            KeyCode::Char('k') | KeyCode::Up => self.move_selection_up(),
            KeyCode::Char('g') => {
                self.selected = 0;
                self.status_message = "Moved to the first task.".to_string();
            }
            KeyCode::Char('G') => {
                self.selected = self.tasks.len().saturating_sub(1);
                self.status_message = "Moved to the last task.".to_string();
            }
            KeyCode::Char('s') => self.cycle_selected_status(),
            KeyCode::Char('p') => self.cycle_selected_priority(),
            KeyCode::Char('?') => self.show_help = true,
            _ => {}
        }
    }

    fn move_selection_down(&mut self) {
        if !self.tasks.is_empty() {
            self.selected = (self.selected + 1).min(self.tasks.len() - 1);
        }
    }

    fn move_selection_up(&mut self) {
        self.selected = self.selected.saturating_sub(1);
    }

    fn cycle_selected_status(&mut self) {
        if let Some(task) = self.tasks.get_mut(self.selected) {
            task.status = task.status.next();
            self.status_message = format!("Task status changed to {}.", task.status);
        }
    }

    fn cycle_selected_priority(&mut self) {
        if let Some(task) = self.tasks.get_mut(self.selected) {
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

fn render(frame: &mut ratatui::Frame<'_>, app: &App) {
    let area = frame.area();

    if app.show_help {
        let help = Paragraph::new(vec![
            Line::from(Span::styled(
                "Keyboard shortcuts",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("j / ↓", Style::default().fg(Color::Cyan)),
                Span::raw("       Move down"),
            ]),
            Line::from(vec![
                Span::styled("k / ↑", Style::default().fg(Color::Cyan)),
                Span::raw("       Move up"),
            ]),
            Line::from(vec![
                Span::styled("g", Style::default().fg(Color::Cyan)),
                Span::raw("           Jump to first task"),
            ]),
            Line::from(vec![
                Span::styled("G", Style::default().fg(Color::Cyan)),
                Span::raw("           Jump to last task"),
            ]),
            Line::from(vec![
                Span::styled("s", Style::default().fg(Color::Cyan)),
                Span::raw("           Cycle task status"),
            ]),
            Line::from(vec![
                Span::styled("p", Style::default().fg(Color::Cyan)),
                Span::raw("           Cycle task priority"),
            ]),
            Line::from(vec![
                Span::styled("?", Style::default().fg(Color::Cyan)),
                Span::raw("           Close this help"),
            ]),
            Line::from(vec![
                Span::styled("q / Esc", Style::default().fg(Color::Cyan)),
                Span::raw("     Quit TaskFlow"),
            ]),
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

    let header = Paragraph::new(Line::from(vec![
        Span::styled(" TaskFlow ", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("Local-first task manager"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(62), Constraint::Percentage(38)])
        .split(sections[1]);

    let total = app.tasks.len();
    let done = app
        .tasks
        .iter()
        .filter(|task| task.status == TaskStatus::Done)
        .count();
    let active = total - done;

    let task_items: Vec<ListItem<'_>> = app
        .tasks
        .iter()
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

    let tasks = List::new(task_items)
        .block(
            Block::default()
                .title(format!(" Tasks ({total}) "))
                .borders(Borders::ALL),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("» ");

    let mut list_state = ListState::default();
    if !app.tasks.is_empty() {
        list_state.select(Some(app.selected));
    }
    frame.render_stateful_widget(tasks, panels[0], &mut list_state);

    let details = if let Some(task) = app.tasks.get(app.selected) {
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
        vec![Line::from("No tasks yet.")]
    };

    let details = Paragraph::new(details)
        .block(Block::default().title(" Details ").borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    frame.render_widget(details, panels[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" j/k ", Style::default().fg(Color::Cyan)),
        Span::raw("Navigate  "),
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

    // Keep the status message visible in the terminal title area without
    // taking space away from the task list.
    let _ = (active, &app.status_message);
}
