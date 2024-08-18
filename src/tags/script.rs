use std::{borrow::Cow, fmt::Display};

#[derive(Debug, PartialEq, Eq)]
pub struct TagScript<'a> {
    // TODO
    // tag:
    contents: Cow<'a, str>,
}

impl Display for TagScript<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<script>\n{}\n</script>", self.contents)
    }
}
impl<'a> TagScript<'a> {
    pub fn new<S: AsRef<str>>(script: S) -> TagScript<'a> {
        Self {
            contents: Cow::from(script.as_ref().to_owned()),
        }
    }
}

impl From<String> for TagScript<'_> {
    fn from(value: String) -> Self {
        Self {
            contents: value.into(),
        }
    }
}

pub fn script<'a, S: AsRef<str>>(script: S) -> TagScript<'a> {
    TagScript::new(script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ok_on_build_simple_script() {
        let script = script(
            format!(
                r#"console.log("Hello from file {} at line {}")"#,
                file!(),
                line!(),
            )
            .as_str(),
        );
        //dbg!(&script);
        println!("{script}");
    }
}
