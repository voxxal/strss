use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, NaiveDateTime};
use rss::{Channel, Item};

use crate::fetch_feed;

pub enum Page {
    Feed(String),
    Article,
}

pub struct State {
    pub scroll: u16,
    pub page: Page,
    pub feeds: HashMap<String, Feed>,
}

pub struct Feed {
    pub id: String,
    pub name: String,
    pub channels: Vec<Channel>,
    pub items: Vec<Item>,
}

impl Feed {
    pub fn new(id: String, urls: Vec<&str>) -> Self {
        let mut feed = Feed {
            id: id.clone(),
            name: id,
            channels: urls
                .into_iter()
                .map(fetch_feed)
                .filter_map(Result::ok)
                .collect(),
            items: vec![],
        };

        feed.rebuild_items();

        feed
    }

    fn rebuild_items(&mut self) {
        let mut items_unflattened = vec![];
        for channel in &self.channels {
            items_unflattened.push(channel.items().to_vec());
        }

        let mut items = items_unflattened
            .into_iter()
            .flatten()
            .collect::<Vec<Item>>();
            
        items.sort_by(|a, b| {
            DateTime::parse_from_rfc2822(
                a.pub_date()
                    .unwrap_or("Thursday, January 1, 1970 12:00:00 AM GMT"),
            )
            .unwrap()
            .cmp(
                &DateTime::parse_from_rfc2822(
                    b.pub_date()
                        .unwrap_or("Thursday, January 1, 1970 12:00:00 AM GMT"),
                )
                .unwrap(),
            )
        });

        self.items = items;
    }

    pub fn add(&mut self, url: &str) -> Result<()> {
        self.channels.push(fetch_feed(url)?);

        Ok(())
    }
}

impl State {
    pub fn new() -> Self {
        let id = String::from("reading"); //who should own the id?
        let mut state = Self {
            scroll: 0,
            page: Page::Feed(id.clone()),
            feeds: HashMap::new(),
        };


        state.feeds.insert(
            id.clone(),
            Feed::new(id, vec!["https://experimentalhistory.substack.com/feed.xml", "https://xkcd.com/rss.xml"])
        );

        state
    }

    pub fn on_tick(&self) {}

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.checked_sub(1).unwrap_or(0);
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }
}
