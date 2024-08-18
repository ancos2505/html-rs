use html_rs::{Body, HeadItem, Html, Script, Style};

fn main() {
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
    let html = Html::new()
        .head(title)
        .add_style(style)
        .add_script(script2)
        .body(body);

    #[cfg(feature = "debug")]
    dbg!(&html);

    println!("{html}");
}
