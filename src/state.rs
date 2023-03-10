use std::{cmp::Reverse, collections::HashMap};

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use rss::{Channel, Guid, Item, Source};

use crate::fetch_feed;

pub enum Page<'a> {
    Empty,
    Feed(&'a str),
    Article(Item),
}

pub enum PageState<'a> {
    Empty,
    Feed { id: &'a str, feed: Feed },
    Article { id: Guid, item: Item },
}

pub struct State<'a> {
    pub scroll: u16,
    pub page: Page<'a>,
    pub page_state: PageState<'a>,
    pub feeds: HashMap<String, Feed>, // TODO move to Feeds type
}

#[derive(Clone)]
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

        items.sort_by_cached_key(|i| {
            Reverse(
                i.pub_date()
                    .map(DateTime::parse_from_rfc2822)
                    .transpose()
                    .ok()
                    .flatten()
                    .unwrap_or(DateTime::<FixedOffset>::MIN_UTC.into()),
            )
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
            page_state: PageState::Empty,
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
        match page {
            Page::Empty => self.page_state = PageState::Empty,
            Page::Feed(id) => match self.feeds.get(id) {
                Some(feed) => {
                    self.page_state = PageState::Feed {
                        id,
                        feed: (*feed).clone(),
                    }
                }
                None => self.page_state = PageState::Empty,
            },
            Page::Article(ref item) => {
                self.page_state = PageState::Article {
                    item: item.clone(),
                    id: item.guid.clone().unwrap(),
                }
            }
        }
        self.scroll = 0;
        self.page = page;
    }

    pub fn on_tick(&self) {}

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll += 1;
    }
}
