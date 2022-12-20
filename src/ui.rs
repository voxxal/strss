use std::io;

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use html2text::{from_read_with_decorator, render::text_renderer::RichDecorator};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap},
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

    let title = state
        .channel
        .as_ref()
        .map(|x| x.title.clone())
        .unwrap_or(String::from("strss"));

    let paragraph = Paragraph::new(Span::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::BOTTOM))
    .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    let content = state
        .channel
        .as_ref()
        .map(|x| x.items[0].content.clone())
        .flatten()
        .unwrap_or(String::from(""));

    let buf: &[u8] = content.as_bytes();

    let paragraph = Paragraph::new(Span::from(from_read_with_decorator(
        buf,
        usize::MAX,
        RichDecorator::new(),
    )))
    .wrap(Wrap { trim: true })
    .scroll((state.scroll, 0));
    f.render_widget(paragraph, chunks[1]);
}
