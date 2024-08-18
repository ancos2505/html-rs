use std::fmt::Display;

use super::{body::TagBody, head::TagHeadLike, script::TagScript, style::TagStyle};

pub fn html<'a>() -> TagHtml<'a> {
    TagHtml::new()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TagHtml<'a> {
    head: Vec<TagHeadLike<'a>>,
    styles: Vec<TagStyle<'a>>,
    scripts: Vec<TagScript<'a>>,
    body: Option<TagBody<'a>>,
}
impl Display for TagHtml<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let head = {
            let mut inner = "<head>".to_owned();
            for head in &self.head {
                inner.push_str(format!("\n{}", head).as_str())
            }
            inner.push_str("\n</head>");
            inner
        };

        let styles = {
            let mut inner = "".to_owned();
            for style in &self.styles {
                inner.push_str(format!("\n{}", style).as_str())
            }
            inner.push_str("\n");
            inner
        };

        let scripts = {
            let mut inner = "".to_owned();
            for script in &self.scripts {
                inner.push_str(format!("\n{}", script).as_str())
            }
            inner.push_str("\n");
            inner
        };

        let body = self
            .body
            .as_ref()
            .map(|body| body.to_string())
            .unwrap_or("".into());

        write!(
            f,
            "<!doctype html>\n<html>\n{head}\n{styles}\n{scripts}\n{body}\n</html>"
        )
    }
}

impl<'a> TagHtml<'a> {
    pub fn new() -> TagHtml<'a> {
        Default::default()
    }
    pub fn head(mut self, head: TagHeadLike<'a>) -> TagHtml<'a> {
        self.head.push(head);
        TagHtml {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn add_style(mut self, style: TagStyle<'a>) -> TagHtml<'a> {
        self.styles.push(style);
        TagHtml {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn add_script(mut self, script: TagScript<'a>) -> TagHtml<'a> {
        self.scripts.push(script);
        TagHtml {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn body(mut self, body: TagBody<'a>) -> TagHtml<'a> {
        self.body = Some(body);
        TagHtml {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn is_complete(&self) -> bool {
        self.head.first().is_some()
            && self.styles.first().is_some()
            && self.scripts.first().is_some()
            && self.body.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_html() {
        let title = TagHeadLike::new("<title>It works!</title>");
        let style = TagStyle::new("body { color: #000000; }");
        let script1 = TagScript::new(
            format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            )
            .as_str(),
        );

        let body = TagBody::new().script(script1);
        let script2 = TagScript::new(
            format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            )
            .as_str(),
        );
        let html = html()
            .head(title)
            .add_style(style)
            .add_script(script2)
            .body(body);
        //dbg!(&html);
        println!("{html}");
    }
}
