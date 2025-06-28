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

const MAIN_CSS: &str = include_str!("../assets/css/main.css");
const OPEN_SANS_CSS: &str = include_str!("../assets/css/open-sans.css");
const PROFILE_CSS: &str = include_str!("../assets/css/profile.css");
const RESUME_CSS: &str = include_str!("../assets/css/resume.css");

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    name: String,
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
    description: Option<String>,
    link: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    skills: Vec<String>,
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
    let default_lang = "fr";
    let resume_directory = PathBuf::from("resume");

    let resume_files = resume_directory
        .read_dir()
        .expect("failed to read resume directory")
        .map(|dir_entry| dir_entry.expect("failed to read resume file").path());

    for path in resume_files {
        let lang = path
            .file_name()
            .and_then(|s| s.to_str())
            .and_then(|s| s.split_once('.'))
            .map(|(lang, _)| lang)
            .unwrap_or_default();

        let config: Config =
            toml::from_str(&read_to_string(&path).expect("failed to read config file"))
                .expect("failed to parse config file");

        let html = format!(
            "<!doctype html>{}",
            html!(
                head!(
                    title!("CV Youn Mélois"),
                    style!(MAIN_CSS, OPEN_SANS_CSS, PROFILE_CSS, RESUME_CSS)
                ),
                body!(main!(profile(&config, lang), resume(&config, lang),),),
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

        std::fs::create_dir_all(lang).expect("failed to create language directory");

        let mut file = File::create(format!("{}/index.html", lang))
            .expect("failed to create {lang}/index.html");
        file.write_all(&minified)
            .expect("failed to write to {lang}/index.html");

        if lang == default_lang {
            std::fs::copy(format!("{}/index.html", lang), "index.html")
                .expect("failed to copy {lang}/index.html to index.html");
        }
    }
}
