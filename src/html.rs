use std::{collections::LinkedList, ops::Deref};

use html2text::{render::{text_renderer::{RichAnnotation, TextDecorator, TaggedLine, RenderLine, TaggedString, BorderHoriz}, Renderer}, html_trace, html_trace_quiet};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};

/// A decorator for use with `TextRenderer` which outputs plain UTF-8 text
/// with no annotations.  Markup is rendered as text characters or footnotes.
#[derive(Clone, Debug)]
pub struct PlainDecorator {
    links: Vec<String>,
}

impl PlainDecorator {
    /// Create a new `PlainDecorator`.
    #[cfg_attr(feature = "clippy", allow(new_without_default_derive))]
    pub fn new() -> PlainDecorator {
        PlainDecorator { links: Vec::new() }
    }
}

impl TextDecorator for PlainDecorator {
    type Annotation = ();

    fn decorate_link_start(&mut self, url: &str) -> (String, Self::Annotation) {
        self.links.push(url.to_string());
        ("[".to_string(), ())
    }

    fn decorate_link_end(&mut self) -> String {
        format!("][{}]", self.links.len())
    }

    fn decorate_em_start(&mut self) -> (String, Self::Annotation) {
        ("*".to_string(), ())
    }

    fn decorate_em_end(&mut self) -> String {
        "*".to_string()
    }

    fn decorate_strong_start(&mut self) -> (String, Self::Annotation) {
        ("**".to_string(), ())
    }

    fn decorate_strong_end(&mut self) -> String {
        "**".to_string()
    }

    fn decorate_strikeout_start(&mut self) -> (String, Self::Annotation) {
        ("".to_string(), ())
    }

    fn decorate_strikeout_end(&mut self) -> String {
        "".to_string()
    }

    fn decorate_code_start(&mut self) -> (String, Self::Annotation) {
        ("`".to_string(), ())
    }

    fn decorate_code_end(&mut self) -> String {
        "`".to_string()
    }

    fn decorate_preformat_first(&mut self) -> Self::Annotation {
        ()
    }
    fn decorate_preformat_cont(&mut self) -> Self::Annotation {
        ()
    }

    fn decorate_image(&mut self, title: &str) -> (String, Self::Annotation) {
        (format!("[{}]", title), ())
    }

    fn header_prefix(&mut self, level: usize) -> String {
        "#".repeat(level) + " "
    }

    fn quote_prefix(&mut self) -> String {
        "> ".to_string()
    }

    fn unordered_item_prefix(&mut self) -> String {
        "* ".to_string()
    }

    fn ordered_item_prefix(&mut self, i: i64) -> String {
        format!("{}. ", i)
    }

    fn finalise(self) -> Vec<TaggedLine<()>> {
        self.links
            .into_iter()
            .enumerate()
            .map(|(idx, s)| TaggedLine::from_string(format!("[{}]: {}", idx + 1, s), &()))
            .collect()
    }

    fn make_subblock_decorator(&self) -> Self {
        PlainDecorator::new()
    }
}


fn to_span(ts: &TaggedString<Vec<RichAnnotation>>) -> Span<'static> {
    let mut style = Style::default();
    for ann in &ts.tag {
        match *ann {
            RichAnnotation::Default => (),
            RichAnnotation::Link(_) => {
                style = style.add_modifier(Modifier::UNDERLINED);
            }
            RichAnnotation::Image => {
                style = style.fg(Color::Blue);
            }
            RichAnnotation::Emphasis => {
                style = style.add_modifier(Modifier::ITALIC);
            }
            RichAnnotation::Strong => {
                style = style.add_modifier(Modifier::BOLD);
            }
            RichAnnotation::Strikeout => {
                style = style.add_modifier(Modifier::CROSSED_OUT);
            }
            RichAnnotation::Code => {
                style = style.add_modifier(Modifier::DIM).bg(Color::Black);
            }
            RichAnnotation::Preformat(is_cont) => {
                if is_cont {
                    style = style.fg(Color::LightMagenta);
                } else {
                    style = style.fg(Color::Magenta);
                }
            }
        }
    }
    Span::styled(ts.s.clone(), style)
}


pub fn to_spans(annotated: Vec<TaggedLine<Vec<RichAnnotation>>>) -> Vec<Spans<'static>> {
    let mut spans = vec![];
    for line in annotated.iter() {
        let mut line_spans = vec![];
        for ts in line.tagged_strings() {
            line_spans.push(to_span(&ts));
        }
        line_spans.push(Span::from("\n"));
        spans.push(Spans::from(line_spans));
    }
    spans
}