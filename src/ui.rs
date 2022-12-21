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
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};

use crate::state::{Feed, Page, State};

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

    match &state.page {
        Page::Feed(id) => draw_feed(f, state, chunks, state.feeds.get(id).unwrap()), // TODO do a 404 or smth
        _ => (),
    }
}

fn draw_feed<B: Backend>(f: &mut Frame<B>, state: &State, chunks: Vec<Rect>, feed: &Feed) {
    let title = &feed.name;
    let paragraph = Paragraph::new(Span::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::BOTTOM))
    .alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);

    let items: Vec<ListItem> = feed
        .items
        .iter()
        .map(|i| ListItem::new(Span::from(i.title().unwrap_or("Untitled"))))
        .collect();

    let list = List::new(items)
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    f.render_widget(list, chunks[1]);
}
