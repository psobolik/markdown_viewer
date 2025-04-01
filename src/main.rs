mod app;

use std::path::PathBuf;
use dioxus::prelude::*;

#[derive(Clone)]
struct CommandLineArgs(PathBuf);

#[cfg(feature = "desktop")]
#[cfg(not(debug_assertions))]
fn main() {
    // markdown_file() doesn't return unless a Markdown file parameter was specified and the file
    // exists. That means that the app isn't launched unless there's something to open.
    let markdown_file = markdown_file();
    LaunchBuilder::new()
        .with_cfg(dioxus::desktop::Config::new()
            .with_menu(None)
            .with_disable_context_menu(false)
        )
        .with_context_provider(move || Box::new(CommandLineArgs(markdown_file.clone()).clone()))
        .launch(app::App);
}

#[cfg(feature = "desktop")]
#[cfg(debug_assertions)]
fn main() {
    // markdown_file() doesn't return unless a Markdown file parameter was specified and the file
    // exists. That means that the app isn't launched unless there's something to open.
    let markdown_file = markdown_file();
    LaunchBuilder::new()
        .with_context_provider(move || Box::new(CommandLineArgs(markdown_file.clone()).clone()))
        .launch(app::App);
}

fn markdown_file() -> PathBuf {
    // Custom argument parser; returns value as PathBuf, but only if the file exists
    fn value_parser(value: String) -> Result<PathBuf, String> {
        let path_buf = PathBuf::from(value);
        if std::path::Path::exists(&path_buf) {
            Ok(path_buf)
        } else {
            Err(String::from("File does not exist"))
        }
    }
    const MARKDOWN_FILE_ID: &str = "MARKDOWN_FILE";

    let matches = clap::Command::new("Markdown Viewer")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple Markdown file viewer")
        .arg(
            clap::Arg::new(MARKDOWN_FILE_ID)
                .help("Markdown file to view")
                .value_parser(move |value: &str| value_parser(value.to_string()))
                .required(true)
        ).get_matches();
    matches.get_one::<PathBuf>(MARKDOWN_FILE_ID).unwrap().to_path_buf()
}