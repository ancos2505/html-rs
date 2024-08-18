use std::fmt::Display;

use crate::elements::HtmlElement;

use super::script::TagScript;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TagBody<'a> {
    // TODO
    // tag:
    contents: Vec<HtmlElement<'a>>,
    script: Vec<TagScript<'a>>,
}

impl Display for TagBody<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO
        // tag:
        let mut output = "<body>".to_owned();
        for element in &self.contents {
            output.push_str(format!("\n{}", element).as_str());
        }
        for script in &self.script {
            output.push_str(format!("\n{}", script).as_str());
        }
        output.push_str("</body>");

        write!(f, "{output}")
    }
}

pub fn body<'a>() -> TagBody<'a> {
    TagBody::new()
}

impl<'a> TagBody<'a> {
    pub fn new() -> TagBody<'a> {
        Default::default()
    }
    pub fn script(mut self, script: TagScript<'a>) -> TagBody<'a> {
        self.script.push(script);
        TagBody {
            script: self.script,
            contents: self.contents,
        }
    }
    pub fn contents(mut self, element: HtmlElement<'a>) -> TagBody<'a> {
        self.contents.push(element);
        TagBody {
            script: self.script,
            contents: self.contents,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_body() {
        let body = body().script(
            format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            )
            .into(),
        );
        //dbg!(&body);
        println!("{body}");
    }
}
