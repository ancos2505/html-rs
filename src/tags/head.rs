use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Head<'a> {
    contents: Vec<HeadItem<'a>>,
}
impl Display for Head<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        for tag in &self.contents {
            output.push_str(tag.to_string().as_str())
        }
        write!(f, "{output}")
    }
}

impl<'a> Head<'a> {
    pub fn new() -> Head<'a> {
        Default::default()
    }
    pub fn add(mut self, tag: HeadItem<'a>) -> Head<'a> {
        self.contents.push(tag);
        Self {
            contents: self.contents,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct HeadItem<'a> {
    contents: Cow<'a, str>,
}

impl Display for HeadItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl<'a> HeadItem<'a> {
    pub fn new<S: AsRef<str>>(tag_str: S) -> HeadItem<'a> {
        Self {
            contents: Cow::from(tag_str.as_ref().to_owned()),
        }
    }
}

pub fn head<'a>() -> Head<'a> {
    Head::new()
}

#[cfg(test)]
mod tests {
    use crate::tags::head;

    use super::*;

    #[test]
    fn ok_on_build_simple_head() {
        let tag = HeadItem::new("<title>Some title</title>");
        let head = head().add(tag);

        //dbg!(&head);
        println!("{head}");
    }
}
