mod elements;
mod tags;
pub use crate::{
    elements::{Div, HtmlElement, TextContent, P},
    tags::{Body, HeadItem, Html, Script, Style, Tag, TagAttribute},
};

fn ok() {
    let div = Div::builder().attr("class", "light-theme").contents(
        P::builder()
            .attr("class", "light-theme")
            .contents(TextContent::text("It Works!")),
    );
    //dbg!(&div);
    println!("{div}");
}
