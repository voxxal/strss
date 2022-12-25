use html2text::render::text_renderer::{RichAnnotation, TextDecorator, TaggedLine};
use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};
// TODO make my own annotation set, with blockquotes and such

fn to_style(tag: &Vec<RichAnnotation>) -> Style {
    let mut style = Style::default();
    for ann in tag {
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
    style
}


pub fn to_spans(annotated: Vec<TaggedLine<Vec<RichAnnotation>>>) -> Vec<Spans<'static>> {
    let mut spans = vec![];
    for line in annotated.iter() {
        let mut line_spans = vec![];
        for ts in line.tagged_strings() {
            let style = to_style(&ts.tag);
            line_spans.push(Span::styled(ts.s.clone(), style));
        }
        line_spans.push(Span::from("\n"));
        eprintln!("{:?}", line_spans);
        spans.push(Spans::from(line_spans));
    }
    spans
}