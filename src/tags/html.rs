use std::fmt::Display;

use super::{body::Body, head::HeadItem, script::Script, style::Style};

pub fn html<'a>() -> Html<'a> {
    Html::new()
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Html<'a> {
    head: Vec<HeadItem<'a>>,
    styles: Vec<Style<'a>>,
    scripts: Vec<Script<'a>>,
    body: Option<Body<'a>>,
}
impl Display for Html<'_> {
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
            "<!doctype html>\n<html>\n{head}{styles}{scripts}{body}\n</html>"
        )
    }
}

impl<'a> Html<'a> {
    pub fn new() -> Html<'a> {
        Default::default()
    }
    pub fn head(mut self, head: HeadItem<'a>) -> Html<'a> {
        self.head.push(head);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn add_style(mut self, style: Style<'a>) -> Html<'a> {
        self.styles.push(style);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn add_script(mut self, script: Script<'a>) -> Html<'a> {
        self.scripts.push(script);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn body(mut self, body: Body<'a>) -> Html<'a> {
        self.body = Some(body);
        Html {
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
        let title = HeadItem::new("<title>It works!</title>");
        let style = Style::new("body { color: #000000; }");
        let script1 = Script::new(
            format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            )
            .as_str(),
        );

        let body = Body::new().script(script1);
        let script2 = Script::new(
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
