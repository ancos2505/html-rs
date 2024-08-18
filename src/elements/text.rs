use crate::tags::Tag;

use super::HtmlElement;

#[derive(Debug, PartialEq, Eq)]
pub struct TextContent;

impl<'a> TextContent {
    pub fn text<S: AsRef<str>>(text: S) -> HtmlElement<'a> {
        let tag = Tag {
            name: "".into(),
            attrs: Default::default(),
        };
        let text_content = HtmlElement::builder(tag).text(text);
        text_content
    }
}
