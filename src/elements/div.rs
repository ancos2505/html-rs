use crate::tags::HtmlTag;

use super::HtmlElement;

#[derive(Debug, PartialEq, Eq)]
pub struct Div;
impl Div {
    pub const fn as_str() -> &'static str {
        "div"
    }
}

impl<'a> Div {
    pub fn builder() -> HtmlElement<'a> {
        let tag = HtmlTag {
            name: Self::as_str().into(),
            attrs: Default::default(),
        };
        HtmlElement::builder(tag)
    }
}
