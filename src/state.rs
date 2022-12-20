use rss::Channel;

use crate::fetch_feed;

pub struct State {
    pub scroll: u16,
    pub channel: Option<Channel>,
}

impl State {
    pub fn new() -> Self {
        Self {
            scroll: 0,
            channel: fetch_feed("https://experimentalhistory.substack.com/feed.xml").ok(),
        }
    }

    pub fn on_tick(&self) {}

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.checked_sub(1).unwrap_or(0);
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }
}
