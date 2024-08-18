use std::fmt::Display;

use super::{
    body::HtmlBody,
    head::{HtmlHead, HtmlHeadItem},
    script::HtmlScript,
    style::HtmlStyle,
};

pub fn html<'a>() -> Html<'a> {
    Html::new()
}

#[derive(Debug, Default)]
pub struct Html<'a> {
    head: HtmlHead<'a>,
    styles: Vec<HtmlStyle<'a>>,
    scripts: Vec<HtmlScript<'a>>,
    body: Option<HtmlBody<'a>>,
}
impl Display for Html<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let head = format!("{}", self.head);

        let styles = {
            let mut inner = "".to_owned();
            for style in &self.styles {
                inner.push_str(format!("{}", style).as_str())
            }
            inner
        };

        let scripts = {
            let mut inner = "".to_owned();
            for script in &self.scripts {
                inner.push_str(format!("{}", script).as_str())
            }
            inner
        };

        let body = self
            .body
            .as_ref()
            .map(|body| body.to_string())
            .unwrap_or("".into());
        let mut output = "".to_string();
        let open_html = "<!doctype html>\n<html>";
        output.push_str(open_html);
        output.push_str(&head);
        output.push_str(&styles);
        output.push_str(&scripts);
        output.push_str(&body);
        let close_html = "\n<html>";
        output.push_str(&close_html);
        write!(f, "{output}")
    }
}

impl<'a> Html<'a> {
    pub fn new() -> Html<'a> {
        Default::default()
    }
    pub fn head(mut self, head: HtmlHeadItem<'a>) -> Html<'a> {
        self.head.items.push(head);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn add_style(mut self, style: HtmlStyle<'a>) -> Html<'a> {
        self.styles.push(style);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn add_script(mut self, script: HtmlScript<'a>) -> Html<'a> {
        self.scripts.push(script);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn body(mut self, body: HtmlBody<'a>) -> Html<'a> {
        self.body = Some(body);
        Html {
            head: self.head,
            styles: self.styles,
            scripts: self.scripts,
            body: self.body,
        }
    }
    pub fn is_complete(&self) -> bool {
        self.head.items.first().is_some()
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
        let title = HtmlHeadItem::new("<title>It works!</title>");
        let style = HtmlStyle::new("body { color: #000000; }");
        let script1 = HtmlScript::new(
            format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            )
            .as_str(),
        );

        let body = HtmlBody::new().script(script1);
        let script2 = HtmlScript::new(
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
