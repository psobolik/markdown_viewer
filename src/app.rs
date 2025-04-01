use crate::CommandLineArgs;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    fn format_markdown(markdown: &str) -> String {
        let parser = pulldown_cmark::Parser::new_ext(markdown, pulldown_cmark::Options::all());
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        html
    }

    let command_line_args = use_context::<CommandLineArgs>();

    let file = command_line_args.0.canonicalize()?;
    rsx! {
        document::Title { "Markdown Viewer - {file.display()}" }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        match std::fs::read_to_string(&file) {
            Ok(contents) => {
                let html = format_markdown(contents.as_str());
                rsx! { div { id: "viewer", dangerous_inner_html: html } }
            }
            Err(error) => {
                rsx! { p { id: "error-message", {format!("Error reading file: {error}")} } }
            }
        }
    }
}
