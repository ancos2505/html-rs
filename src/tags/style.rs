use std::{borrow::Cow, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct Style<'a> {
    contents: Cow<'a, str>,
}

impl Display for Style<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<style>\n{}\n</style>", self.contents)
    }
}

impl<'a> Style<'a> {
    pub fn new<S: AsRef<str>>(css: S) -> Style<'a> {
        Self {
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
