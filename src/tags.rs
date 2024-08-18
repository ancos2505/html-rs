use std::{borrow::Cow, fmt::Display};

mod body;
mod head;
mod html;
mod script;
mod style;

#[derive(Debug, PartialEq, Eq)]
pub struct HtmlTag<'a> {
    pub name: Cow<'a, str>,
    pub attrs: Vec<HtmlAttribute<'a>>,
}
impl Display for HtmlTag<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = format!("{} ", self.name);
        let max_idx = self.attrs.len();
        for (idx, attr) in self.attrs.iter().enumerate() {
            output.push_str(format!("{attr}").as_str());
            if idx < max_idx {
                output.push(' ')
            }
        }
        write!(f, "{output}")
    }
}
impl HtmlTag<'_> {
    pub fn set_attr<K: AsRef<str>, V: AsRef<str>>(&mut self, key: K, value: V) {
        let attr = HtmlAttribute::from((key.as_ref().to_owned(), value.as_ref().to_owned()));
        self.attrs.push(attr);
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct HtmlAttribute<'a> {
    // TODO
    pub name: Cow<'a, str>,
    pub value: String,
}
impl Display for HtmlAttribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = format!("{}={}", self.name, self.value);
        write!(f, "{output}")
    }
}

impl From<(String, String)> for HtmlAttribute<'_> {
    fn from((key, value): (String, String)) -> Self {
        HtmlAttribute {
            name: key.into(),
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Todo"]
    fn ok_on_build_html_tag() {
        todo!()
    }
}
