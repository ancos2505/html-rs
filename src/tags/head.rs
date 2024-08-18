use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TagHead<'a> {
    contents: Vec<TagHeadLike<'a>>,
}
impl Display for TagHead<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        for tag in &self.contents {
            output.push_str(tag.to_string().as_str())
        }
        write!(f, "{output}")
    }
}

impl<'a> TagHead<'a> {
    pub fn new() -> TagHead<'a> {
        Default::default()
    }
    pub fn add(mut self, tag: TagHeadLike<'a>) -> TagHead<'a> {
        self.contents.push(tag);
        Self {
            contents: self.contents,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TagHeadLike<'a> {
    contents: Cow<'a, str>,
}

impl Display for TagHeadLike<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.contents)
    }
}

impl<'a> TagHeadLike<'a> {
    pub fn new<S: AsRef<str>>(tag_str: S) -> TagHeadLike<'a> {
        Self {
            contents: Cow::from(tag_str.as_ref().to_owned()),
        }
    }
}

pub fn head<'a>() -> TagHead<'a> {
    TagHead::new()
}

#[cfg(test)]
mod tests {
    use crate::tags::head;

    use super::*;

    #[test]
    fn ok_on_build_simple_head() {
        let tag = TagHeadLike::new("<title>Some title</title>");
        let head = head().add(tag);

        //dbg!(&head);
        println!("{head}");
    }
}
