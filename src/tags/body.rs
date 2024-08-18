use std::fmt::Display;

use crate::elements::HtmlElement;

use super::script::Script;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Body<'a> {
    // TODO
    // tag:
    contents: Vec<HtmlElement<'a>>,
    script: Vec<Script<'a>>,
}

impl Display for Body<'_> {
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
        output.push_str("\n</body>");

        write!(f, "{output}")
    }
}

pub fn body<'a>() -> Body<'a> {
    Body::new()
}

impl<'a> Body<'a> {
    pub fn new() -> Body<'a> {
        Default::default()
    }
    pub fn script(mut self, script: Script<'a>) -> Body<'a> {
        self.script.push(script);
        Body {
            script: self.script,
            contents: self.contents,
        }
    }
    pub fn contents(mut self, element: HtmlElement<'a>) -> Body<'a> {
        self.contents.push(element);
        Body {
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
