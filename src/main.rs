mod common;
mod profile;
mod resume;

use std::fs::{
    File,
    read_to_string,
};
use std::io::Write;
use std::path::PathBuf;

use dioxus::prelude::*;
use dioxus::ssr::render_element;
use minify_html::{
    Cfg,
    minify,
};

use crate::profile::profile;
use crate::resume::resume;

fn main() {
    let assets_directory = PathBuf::from("assets");
    let css_directory = assets_directory.join("css");

    let css_files = css_directory
        .read_dir()
        .expect("failed to read css directory")
        .map(|dir_entry| dir_entry.expect("failed to read css file").path());

    let css_content = css_files
        .map(|path| read_to_string(path).expect("failed to read css file"))
        .collect::<String>();

    let html_content = render_element(rsx! {
        main {
            {profile()}
            {resume()}
        }
    });

    let html = format!(
        "<!doctype html>
        <html>
            <head>
                <title>Youn Mélois</title>
                <style>{css_content}</style>
            </head>
            <body>
                {html_content}
            </body>
        </html>",
    );

    let mut cfg = Cfg::default();

    cfg.allow_optimal_entities = true;
    cfg.allow_removing_spaces_between_attributes = true;
    cfg.minify_doctype = true;
    cfg.minify_css = true;
    cfg.remove_bangs = true;
    cfg.remove_processing_instructions = true;

    let minified = minify(html.as_bytes(), &cfg);

    let mut file = File::create("index.html").expect("failed to create index.html");
    file.write_all(&minified)
        .expect("failed to write to index.html");
}
