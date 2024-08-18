use std::fmt::Display;

use crate::{elements::HtmlElement, OUTPUT_IDENTATION};

use super::{script::Script, Tag};

#[derive(Debug, PartialEq, Eq)]
pub struct Body<'a> {
    tag: Tag<'a>,
    depth: usize,
    elements: Vec<HtmlElement<'a>>,
    script: Vec<Script<'a>>,
}

impl Default for Body<'_> {
    fn default() -> Self {
        Self {
            tag: Tag {
                name: Self::as_str().into(),
                attrs: Default::default(),
            },
            depth: 1,
            elements: Default::default(),
            script: Default::default(),
        }
    }
}

impl Display for Body<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);
        output.push_str(format!("\n{iden}<{}>", self.tag).as_str());
        for elem in &self.elements {
            output.push_str(format!("{iden}{}", elem).as_str());
        }
        output.push_str(format!("\n{iden}</{}>", self.tag.name).as_str());
        write!(f, "{output}")
    }
}

pub fn body<'a>() -> Body<'a> {
    Body::new()
}

impl<'a> Body<'a> {
    pub const fn as_str() -> &'static str {
        "body"
    }
    pub fn new() -> Body<'a> {
        Default::default()
    }
    pub fn script(mut self, script: Script<'a>) -> Body<'a> {
        self.script.push(script);
        Body {
            tag: self.tag,
            depth: self.depth,
            script: self.script,
            elements: self.elements,
        }
    }
    pub fn append_child(mut self, mut element: HtmlElement<'a>) -> Body<'a> {
        element.set_depth(self.depth + 1);
        self.elements.push(element);
        Body {
            tag: self.tag,
            depth: self.depth,
            script: self.script,
            elements: self.elements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_body() {
        let body = body().script(
            Script::new(format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            ))
            .into(),
        );
        //dbg!(&body);
        println!("{body}");
    }
}
