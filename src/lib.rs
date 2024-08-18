pub mod elements;
mod tags;

pub const OUTPUT_IDENTATION: usize = 4; // Spaces

pub use crate::tags::{Body, Head, HeadItem, Html, Script, Style, Tag, TagAttribute};
