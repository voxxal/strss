use std::io;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

use crate::state::State;

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    Ok(Terminal::new(backend)?)
}

pub fn restore_terminal(mut terminal: Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    enable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, state: &State) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(size);

    let paragraph = Paragraph::new(Span::styled(
        "Thing",
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::BOTTOM))
    .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    let paragraph = Paragraph::new(Span::from(state.scroll.to_string()));
    f.render_widget(paragraph, chunks[1]);
}
