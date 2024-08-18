use crate::tags::Tag;

use super::HtmlElement;

#[derive(Debug, PartialEq, Eq)]
pub struct P;

impl P {
    pub const fn as_str() -> &'static str {
        "p"
    }
}
impl<'a> P {
    pub fn builder() -> HtmlElement<'a> {
        let tag = Tag {
            name: Self::as_str().into(),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
