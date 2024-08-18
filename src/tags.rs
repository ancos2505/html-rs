mod body;
mod head;
mod html;
mod script;
mod style;

use std::{borrow::Cow, fmt::Display};

pub use self::{
    body::Body,
    head::{Head, HeadItem},
    html::Html,
    script::Script,
    style::Style,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Tag<'a> {
    pub name: Cow<'a, str>,
    pub attrs: Vec<TagAttribute<'a>>,
}
impl Display for Tag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("{}", self.name);
        let max_idx = self.attrs.len();
        if max_idx > 0 {
            output.push(' ');
        }
        for (idx, attr) in self.attrs.iter().enumerate() {
            output.push_str(format!("{attr}").as_str());
            if idx + 1 < max_idx {
                output.push(' ')
            }
        }
        write!(f, "{output}")
    }
}
impl Tag<'_> {
    pub fn set_attr<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        let attr = TagAttribute::from((key.as_ref().to_owned(), value.as_ref().to_owned()));
        self.attrs.push(attr);
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct TagAttribute<'a> {
    // TODO
    pub name: Cow<'a, str>,
    pub value: String,
}
impl Display for TagAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!(r#"{}="{}""#, self.name, self.value);
        write!(f, "{output}")
    }
}

impl From<(String, String)> for TagAttribute<'_> {
    fn from((key, value): (String, String)) -> Self {
        TagAttribute {
            name: key.into(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    #[ignore = "Todo"]
    fn ok_on_build_html_tag() {
        todo!()
    }
}
