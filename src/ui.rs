use std::io;

use anyhow::Result;
use chrono::DateTime;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use html2text::{from_read_with_decorator, render::text_renderer::RichDecorator};
use rss::Source;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
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
    // TODO the header shouldn't have the horizontal margin
    let chunks = Layout::default()
        .horizontal_margin(20)
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)].as_ref())
        .split(size);
    match state.page {
        Page::Feed(id) => match state.feeds.get(id) {
            Some(feed) => draw_feed(f, state, chunks, feed),
            None => draw_not_found(f, state, chunks), // TODO do a 404 or smth
        },
        _ => (),
    }
}

fn draw_not_found<B: Backend>(f: &mut Frame<B>, state: &State, chunks: Vec<Rect>) {
    f.render_widget(title_widget("strss"), chunks[0]);

    let content = Paragraph::new(Span::from("That feed doesn't exist."));

    f.render_widget(content, chunks[1])
}

fn title_widget(title: &str) -> Paragraph {
    Paragraph::new(Span::styled(
        title,
        Style::default().add_modifier(Modifier::BOLD),
    ))
    .block(Block::default().borders(Borders::BOTTOM))
    .alignment(Alignment::Center)
}

fn draw_feed<B: Backend>(f: &mut Frame<B>, state: &State, chunks: Vec<Rect>, feed: &Feed) {
    f.render_widget(title_widget(&feed.name), chunks[0]);

    let items: Vec<Spans> = feed
        .items
        .iter()
        .map(|i| {
            let mut portions = vec![
                Spans::from(Span::styled(
                    i.title().unwrap_or("Untitled"),
                    Style::default().add_modifier(Modifier::BOLD),
                )),
                Spans::from(vec![Span::styled(
                    i.source().map(Source::title).flatten().unwrap_or("Unknown"),
                    Style::default().fg(Color::DarkGray),
                )]),
            ];

            if let Some(pub_date) = i.pub_date() {
                if let Ok(date) = DateTime::parse_from_rfc2822(pub_date) {
                    portions[1]
                        .0
                        .push(Span::styled(" | ", Style::default().fg(Color::DarkGray)));

                    let formatted = format!("{}", date.format("%Y-%m-%d"));
                    portions[1].0.push(Span::styled(
                        formatted,
                        Style::default().fg(Color::DarkGray),
                    ))
                }
            }

            portions.push(Spans::from(Span::from("\n")));

            portions
        })
        .flatten()
        .collect();

    let content = Paragraph::new(items).scroll((state.scroll, 0));
    f.render_widget(content, chunks[1]);
}
