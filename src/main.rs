use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use reqwest::blocking as reqwest;
use rss::Channel;
use state::{State, Page};
use tui::{backend::Backend, Terminal};

mod state;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ui::setup_terminal()?;

    let res = run(
        &mut terminal,
        State::new(),
        Duration::from_millis(42), /* Around 24fps */
    );

    ui::restore_terminal(terminal)?;

    if let Err(err) = res {
        eprintln!("{:?}", err)
    }

    Ok(())
}

fn run<B: Backend>(
    terminal: &mut Terminal<B>,
    mut state: State,
    tick_rate: Duration,
) -> Result<()> {
    let mut last_tick = Instant::now();
    state.navigate(Page::Feed("reading"));
    loop {
        terminal.draw(|f| ui::draw_ui(f, &state))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => state.scroll_up(),
                    KeyCode::Down => state.scroll_down(),
                    _ => (),
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::ScrollUp => state.scroll_up(),
                    MouseEventKind::ScrollDown => state.scroll_down(),
                    _ => (),
                },
                _ => (),
            }
        }

        if last_tick.elapsed() >= tick_rate {
            state.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn fetch_feed(url: &str) -> Result<Channel> {
    let content = reqwest::get(url)?.bytes()?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
