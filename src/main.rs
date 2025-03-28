#[macro_use]
mod element;
mod link;
mod profile;
mod resume;

use std::fs::{
    File,
    read_to_string,
};
use std::io::Write;
use std::path::PathBuf;

use minify_html::{
    Cfg,
    minify,
};

use crate::element::*;
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

    let html = format!(
        "<!doctype html>{}",
        html!(
            head!(title!("Youn Mélois"), style!(css_content)),
            body!(main!(profile(), resume())),
        )
        .to_string()
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
