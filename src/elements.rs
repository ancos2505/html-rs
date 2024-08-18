mod div;
mod p;
mod text;

use std::{
    borrow::Cow,
    fmt::{Debug, Display},
};

pub use self::{div::Div, p::P, text::TextContent};

use crate::{tags::Tag, OUTPUT_IDENTATION};

pub trait ElementName: Debug {}

#[derive(Debug, PartialEq, Eq)]
pub enum HtmlElementChildren<'a> {
    TextContent(Cow<'a, str>),
    Children(Vec<HtmlElement<'a>>),
}

impl Display for HtmlElementChildren<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            HtmlElementChildren::TextContent(text) => text.to_owned(),
            HtmlElementChildren::Children(html_elements) => {
                let mut output = "".to_owned();
                for elem in html_elements {
                    output.push_str(elem.to_string().as_str())
                }
                output.into()
            }
        };

        write!(f, "{output}")
    }
}

#[derive(Debug)]
pub struct HtmlElement<'a> {
    pub tag: Tag<'a>,
    depth: usize,
    pub children: Option<HtmlElementChildren<'a>>,
}

impl Display for HtmlElement<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = "".to_owned();
        let iden = " ".repeat(OUTPUT_IDENTATION * self.depth);

        let tagname_and_attrs = format!("{}", self.tag);
        let tag_name = &self.tag.name;

        match &self.children {
            Some(children) => {
                if let HtmlElementChildren::TextContent(s) = children {
                    let text_iden = " ".repeat(OUTPUT_IDENTATION * (self.depth + 1));
                    output.push_str(format!("\n{text_iden}{s}").as_str())
                } else {
                    output.push_str(
                        format!("\n{iden}<{tagname_and_attrs}>{children}\n{iden}</{tag_name}>")
                            .as_str(),
                    )
                }
            }
            None => output.push_str(format!("\n{iden}<{tagname_and_attrs}></{tag_name}>").as_str()),
        };

        write!(f, "{output}")
    }
}
impl PartialEq for HtmlElement<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag && self.depth == other.depth && self.children == other.children
        // TODO
        // && self.token == other.token
    }
}

impl Eq for HtmlElement<'_> {}
impl<'a> HtmlElement<'a> {
    pub fn builder(tag: Tag<'a>) -> HtmlElement<'a> {
        HtmlElement {
            tag,
            // It exists from Body(depth=2)
            depth: 2,
            children: Default::default(),
        }
    }
    pub fn text<S: AsRef<str>>(mut self, text: S) -> HtmlElement<'a> {
        self.children = Some(HtmlElementChildren::TextContent(
            text.as_ref().to_owned().into(),
        ));

        HtmlElement {
            tag: self.tag,
            depth: self.depth,
            children: self.children,
        }
    }
    pub fn attr<K: AsRef<str>, V: AsRef<str>>(mut self, key: K, value: V) -> HtmlElement<'a> {
        self.tag.set_attr(key, value);

        HtmlElement {
            tag: self.tag,
            depth: self.depth,
            children: self.children,
        }
    }
    pub fn append_child(self, mut new_element: HtmlElement<'a>) -> HtmlElement<'a> {
        new_element.depth = self.depth + 1;
        dbg!(&new_element);
        if let Some(children) = self.children {
            let new_children = match children {
                HtmlElementChildren::TextContent(text) => {
                    let migrated = TextContent::text(text);
                    HtmlElementChildren::Children(vec![migrated, new_element])
                }
                HtmlElementChildren::Children(mut html_elements) => {
                    html_elements.push(new_element);
                    HtmlElementChildren::Children(html_elements)
                }
            };
            HtmlElement {
                tag: self.tag,
                depth: self.depth,
                children: Some(new_children),
            }
        } else {
            HtmlElement {
                tag: self.tag,
                depth: self.depth,
                children: Some(HtmlElementChildren::Children(vec![new_element])),
            }
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn set_depth(&mut self, depth: usize) {
        self.depth = depth;
    }
}

#[cfg(test)]
mod tests {
    use text::TextContent;

    use super::*;
    use crate::elements::{div::Div, p::P};
    #[test]
    fn ok_on_build_div_with_paragraph() {
        let div = Div::builder().attr("class", "light-theme").append_child(
            P::builder()
                .attr("class", "light-theme")
                .append_child(TextContent::text("It Works!")),
        );

        println!("{div}");
    }
}
