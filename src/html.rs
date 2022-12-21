use html2text::render::text_renderer::TextDecorator;

#[derive(Clone, Debug)]
pub struct TuiDecorator {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TuiAnnotation {
    /// Normal text.
    Default,
    /// A link with the target.
    Link(String),
    /// An image (attached to the title text)
    Image,
    /// Emphasised text, which might be rendered in bold or another colour.
    Emphasis,
    /// Strong text, which might be rendered in bold or another colour.
    Strong,
    /// Stikeout text
    Strikeout,
    /// Code
    Code,
    /// Preformatted; true if a continuation line for an overly-long line.
    Preformat(bool),
}

impl Default for TuiAnnotation {
    fn default() -> Self {
        TuiAnnotation::Default
    }
}

impl TuiDecorator {
    pub fn new() -> TuiDecorator {
        TuiDecorator {}
    }
}

impl TextDecorator for TuiDecorator {
    type Annotation = TuiAnnotation;
}