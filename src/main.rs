use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use reqwest::blocking as reqwest;
use rss::Channel;
use state::{Page, State};
use tui::{backend::Backend, Terminal};

// mod html;
mod state;
mod ui;

fn main() -> Result<()> {
    let mut terminal = ui::setup_terminal()?;

    let res = run(
        &mut terminal,
        Duration::from_millis(42), /* Around 24fps */
    );

    ui::restore_terminal(terminal)?;

    if let Err(err) = res {
        eprintln!("{:?}", err)
    }

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> Result<()> {
    let mut state = State::new();
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
                    KeyCode::Left => state.navigate(Page::Feed("reading")),
                    _ => (),
                },
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => match state.page {
                        Page::Feed(id) => {
                            if mouse.row > 2 {
                                let content_y = mouse.row - 2 + state.scroll;
                                let selection = content_y / 3;
                                if (content_y + 1) % 3 != 0 {
                                    let dest = match &state.feeds.get(id) {
                                        Some(feed) => {
                                            Page::Article(feed.items[selection as usize].clone())
                                        }
                                        None => Page::Empty,
                                    };
                                    state.navigate(dest);
                                }
                            }
                        }
                        Page::Article(_) => {
                            if mouse.column >= 20 && mouse.column <= 26 && mouse.row == 0 {
                                state.navigate(Page::Feed("reading"))
                            }
                        }
                        _ => (),
                    },
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
