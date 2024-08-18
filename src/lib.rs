pub mod elements;
mod tags;

pub const OUTPUT_IDENTATION: usize = 4; // Spaces

pub use crate::tags::{
    Html, HtmlBody, HtmlHead, HtmlHeadItem, HtmlScript, HtmlStyle, Tag, TagAttribute,
};
