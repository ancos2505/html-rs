use std::{borrow::Cow, fmt::Display};

use crate::OUTPUT_IDENTATION;

use super::Tag;

#[derive(Debug, PartialEq, Eq)]
pub struct Head<'a> {
    pub tag: Tag<'a>,
    pub depth: usize,
    pub items: Vec<HeadItem<'a>>,
}
impl Default for Head<'_> {
    fn default() -> Self {
        Self {
            tag: Tag {
                name: Self::as_str().into(),
                attrs: Default::default(),
            },
            depth: 1,
            items: Default::default(),
        }
    }
}
impl Display for Head<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>", self.tag).as_str());
        for item in &self.items {
            output.push_str(format!("\n{}", item).as_str());
        }
        output.push_str(format!("\n{iden}</{}>", self.tag.name).as_str());
        write!(f, "{output}")
    }
}

impl<'a> Head<'a> {
    pub const fn as_str() -> &'static str {
        "head"
    }
    pub fn new() -> Head<'a> {
        Default::default()
    }
    pub fn add(mut self, tag: HeadItem<'a>) -> Head<'a> {
        self.items.push(tag);
        Self {
            tag: self.tag,
            depth: self.depth,
            items: self.items,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HeadItem<'a> {
    depth: usize,
    contents: Cow<'a, str>,
}

impl Display for HeadItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_string();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("{iden}{}", self.contents).as_str());
        write!(f, "{output}")
    }
}

impl<'a> HeadItem<'a> {
    pub fn new<S: AsRef<str>>(tag_str: S) -> HeadItem<'a> {
        Self {
            depth: 2,
            contents: Cow::from(tag_str.as_ref().to_owned()),
        }
    }
}

pub fn head<'a>() -> Head<'a> {
    Head::new()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn ok_on_build_simple_head() {
        let tag = HeadItem::new("<title>Some title</title>");
        let head = head().add(tag);

        //dbg!(&head);
        println!("{head}");
    }
}
