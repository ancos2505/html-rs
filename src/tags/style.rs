use std::{borrow::Cow, fmt::Display};

use crate::OUTPUT_IDENTATION;

use super::Tag;

#[derive(Debug, PartialEq, Eq)]
pub struct Style<'a> {
    tag: Tag<'a>,
    depth: usize,
    contents: Cow<'a, str>,
}

impl Display for Style<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>\n", self.tag).as_str());

        output.push_str(&self.contents);

        output.push_str(format!("\n{iden}</{}>", self.tag.name).as_str());
        write!(f, "{output}")
    }
}

impl<'a> Style<'a> {
    pub const fn as_str() -> &'static str {
        "style"
    }
    pub fn new<S: AsRef<str>>(css: S) -> Style<'a> {
        Self {
            tag: Tag {
                name: Self::as_str().into(),
                attrs: Default::default(),
            },
            depth: 1,
            contents: Cow::from(css.as_ref().to_owned()),
        }
    }
}

pub fn style<'a, S: AsRef<str>>(css: S) -> Style<'a> {
    Style::new(css)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_style() {
        let style = style("body { color: #000000; }");
        //dbg!(&style);
        println!("{style}");
    }
}
