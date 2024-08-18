use std::{borrow::Cow, fmt::Display};

use crate::OUTPUT_IDENTATION;

use super::Tag;

#[derive(Debug, PartialEq, Eq)]
pub struct Script<'a> {
    tag: Tag<'a>,
    depth: usize,
    contents: Cow<'a, str>,
}

impl Display for Script<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>\n", self.tag).as_str());

        output.push_str(&self.contents);

        output.push_str(format!("\n{iden}</{}>", self.tag.name).as_str());
        write!(f, "{output}")
    }
}

impl<'a> Script<'a> {
    pub const fn as_str() -> &'static str {
        "script"
    }
    pub fn new<S: AsRef<str>>(script: S) -> Script<'a> {
        Self {
            tag: Tag {
                name: Self::as_str().into(),
                attrs: Default::default(),
            },
            depth: 1,
            contents: Cow::from(script.as_ref().to_owned()),
        }
    }
}

pub fn script<'a, S: AsRef<str>>(script: S) -> Script<'a> {
    Script::new(script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_script() {
        let script = script("body { color: #000000; }");
        //dbg!(&script);
        println!("{script}");
    }
}
