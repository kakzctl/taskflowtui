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
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

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

#[derive(Default)]
struct App {
    should_quit: bool,
}

fn main() -> io::Result<()> {
    let mut terminal = TerminalGuard::new()?;
    let mut app = App::default();

    run(terminal.terminal(), &mut app)
}

fn run(terminal: &mut AppTerminal, app: &mut App) -> io::Result<()> {
    while !app.should_quit {
        terminal.draw(render)?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                // Ignore key-repeat/release events; handle each press once.
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                match (key.code, key.modifiers) {
                    (KeyCode::Char('q'), _) | (KeyCode::Esc, _) => {
                        app.should_quit = true;
                    }
                    (KeyCode::Char('c'), modifiers)
                        if modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.should_quit = true;
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn render(frame: &mut ratatui::Frame<'_>) {
    let area = frame.area();

    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    let header = Paragraph::new("TaskFlow")
        .style(Style::default().add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));

    let welcome = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Welcome to TaskFlow",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Your tasks. Your projects. Your focus."),
        Line::from(""),
        Line::from("The task dashboard is coming in the next milestone."),
    ])
    .block(
        Block::default()
            .title(" Workspace ")
            .borders(Borders::ALL),
    );

    let footer = Paragraph::new(" q Quit  •  Esc Quit  •  Ctrl+C Quit ")
        .block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, sections[0]);
    frame.render_widget(welcome, sections[1]);
    frame.render_widget(footer, sections[2]);
}
