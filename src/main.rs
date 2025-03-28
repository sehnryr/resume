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
use serde::Deserialize;

use crate::element::*;
use crate::profile::profile;
use crate::resume::resume;

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    name: String,
    picture: String,
    address: String,
    title: String,
    subtitle: String,
    contact: Contact,
    languages: Vec<Language>,
    soft_skills: Vec<SoftSkill>,
    hard_skills: Vec<HardSkill>,
    interests: Vec<Interest>,
    experiences: Vec<Experience>,
    education: Vec<Education>,
    projects: Vec<Project>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Contact {
    email: String,
    phone: String,
    website: String,
    bluesky: String,
    linkedin: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Language {
    name: String,
    level: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SoftSkill {
    name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct HardSkill {
    name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    examples: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Interest {
    name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    examples: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Experience {
    company: String,
    time: String,
    title: String,
    link: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Education {
    school: String,
    degree: String,
    time: String,
    description: Option<String>,
    link: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Project {
    title: String,
    description: String,
    link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    skills: Vec<String>,
}

fn main() {
    let config_path = PathBuf::from("resume.toml");
    let config: Config =
        toml::from_str(&read_to_string(config_path).expect("failed to read config file"))
            .expect("failed to parse config file");

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
            body!(main!(profile(&config), resume(&config),),),
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
