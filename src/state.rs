use std::collections::HashMap;

use anyhow::Result;
use chrono::DateTime;
use rss::{Channel, Item, Source};

use crate::fetch_feed;

pub enum Page<'a> {
    Empty,
    Feed(&'a str),
    Article(&'a Item),
}

pub struct State<'a> {
    pub scroll: u16,
    pub page: Page<'a>,
    pub feeds: HashMap<String, Feed>,
}

pub struct Feed {
    pub id: String,
    pub name: String,
    pub channels: Vec<Channel>,
    pub items: Vec<Item>, // Item should probably hold more info which allows us to not write such terrible code in ui.rs
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
            let mut items = channel.items().to_vec();
            for item in &mut items {
                item.set_source(Source {
                    url: channel.link.clone(),
                    title: Some(channel.title.clone()),
                })
            }

            items_unflattened.push(items);
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
            .reverse()
        });

        self.items = items;
    }

    pub fn add(&mut self, url: &str) -> Result<()> {
        self.channels.push(fetch_feed(url)?);

        Ok(())
    }
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        let id = String::from("reading"); //who should own the id?
        let mut state = Self {
            scroll: 0,
            page: Page::Empty,
            feeds: HashMap::new(),
        };

        state.feeds.insert(
            id.clone(),
            Feed::new(
                id,
                vec![
                    "https://experimentalhistory.substack.com/feed.xml",
                    "https://xkcd.com/rss.xml",
                ],
            ),
        );

        state
    }

    pub fn navigate(&mut self, page: Page<'a>) {
        self.page = page;
    }

    pub fn on_tick(&self) {}

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.checked_sub(1).unwrap_or(0);
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }
}
