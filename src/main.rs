mod common;
mod profile;
mod resume;

use std::fs::File;
use std::io::Write;

use dioxus::prelude::*;
use dioxus::ssr::render_element;
use minify_html::{
    Cfg,
    minify,
};

use crate::profile::profile;
use crate::resume::resume;

const OPEN_SANS_FONT_CSS: &str = include_str!("../assets/css/open-sans.css");
const MAIN_CSS: &str = include_str!("../assets/css/main.css");
const PROFILE_CSS: &str = include_str!("../assets/css/profile.css");
const RESUME_CSS: &str = include_str!("../assets/css/resume.css");

fn main() {
    let content = render_element(rsx! {
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
                <style>{OPEN_SANS_FONT_CSS}</style>
                <style>{MAIN_CSS}</style>
                <style>{PROFILE_CSS}</style>
                <style>{RESUME_CSS}</style>
            </head>
            <body>
                {content}
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
