use dioxus::prelude::*;

use crate::common::{
    LinkType,
    link,
};

const LINK_SVG: &str = include_str!("../assets/icons/link-2.svg");

pub fn resume() -> Element {
    rsx! {
        div { class: "resume",
            h1 { "Ingénieur Logiciel" }
            h2 { "Programmation Temps Réel & Performances" }
            {experiences()}
            {education()}
            {projects()}
        }
    }
}

struct Experience<'a> {
    company: &'a str,
    time: &'a str,
    title: &'a str,
    link: Option<&'a str>,
}

fn experiences() -> Element {
    let experiences = vec![
        Experience {
            company: "Clever Cloud",
            time: "(oct. 2024 - aujourd'hui)",
            title: "Développeur logiciels en Alternance",
            link: Some("clever-cloud.com"),
        },
        Experience {
            company: "Clever Cloud",
            time: "(juil. 2024 - sept. 2024)",
            title: "Développeur logiciels en Stage",
            link: None,
        },
    ];

    rsx! {
        section { class: "experience",
            h2 { "Expériences professionnelles" }
            ul {
                for experience in experiences {
                    li {
                        {experience.company}
                        span { class: "date", {experience.time} }
                        p { {experience.title} }
                        if let Some(link_) = experience.link {
                            div { dangerous_inner_html: LINK_SVG, {link(LinkType::Website, link_)} }
                        }
                    }
                }
            }
        }
    }
}

struct Education<'a> {
    school: &'a str,
    degree: &'a str,
    time: &'a str,
    description: Option<&'a str>,
    link: Option<&'a str>,
}

fn education() -> Element {
    let education = vec![
        Education {
            school: "ISEN Yncréa Ouest",
            degree: "Diplôme d'ingénieur, Ingénierie Logicielle",
            time: "(sept. 2020 - oct. 2025)",
            description: None,
            link: Some("isen-nantes.fr"),
        },
        Education {
            school: "Sveučilište Algebra (Algebra University)",
            degree: "Ingénierie Logicielle",
            time: "(févr. 2024 - juil. 2024)",
            description: Some("Erasmus d'un semèstre à Zagreb en Croatie."),
            link: Some("algebra.hr"),
        },
    ];

    rsx! {
        section { class: "education",
            h2 { "Formation" }
            ul {
                for education in education {
                    li {
                        {education.school}
                        span { class: "date", {education.time} }
                        p { {education.degree} }
                        if let Some(description) = education.description {
                            p { {description} }
                        }
                        if let Some(link_) = education.link {
                            div { dangerous_inner_html: LINK_SVG, {link(LinkType::Website, link_)} }
                        }
                    }
                }
            }
        }
    }
}

struct Project<'a> {
    title: &'a str,
    description: &'a str,
    link: Option<&'a str>,
    skills: Vec<&'a str>,
}

fn projects() -> Element {
    let projects = vec![
        Project {
            title: "Lecteur de manga",
            description: "Création d'une application multiplateforme (mobile/desktop) permettant \
                          de visualiser des mangas/webtoons depuis des sources populaires.",
            link: Some("github.com/midokuapp/midoku"),
            skills: vec!["Rust", "WebAssembly", "Dioxus"],
        },
        Project {
            title: "Réécriture de Oodle en Rust",
            description: "Oodle est un algorithme de compression de données utilisé dans de \
                          nombreux jeux vidéo. Cette réécriture permet d'utiliser Oodle dans des \
                          applications Rust sans avoir à faire du linkage avec la bibliothèque C \
                          originale.",
            link: Some("github.com/sehnryr/oodle-rs"),
            skills: vec!["Rust", "FFI", "Bindgen"],
        },
        Project {
            title: "Rétro-ingénierie du cache de Warframe",
            description: "Extraction des données compressées binaires du jeu Warframe pour en \
                          récupérer les images et musiques. Création d'une application TUI pour \
                          explorer et extraire ces données.",
            link: Some("github.com/sehnryr/wfcache-api"),
            skills: vec!["Rust", "C++", "TUI", "ImHex"],
        },
        Project {
            title: "Contributions Open-source",
            description: "Création d'Issues et Pull Requests sur des projets open-source: dioxus, \
                          rust-bindgen, topgrade, cargo-component",
            link: None,
            skills: vec![],
        },
    ];

    rsx! {
        section { class: "project",
            h2 { "Projets" }
            ul {
                for project in projects {
                    li {
                        {project.title}
                        p { {project.description} }
                        if let Some(link_) = project.link {
                            div { dangerous_inner_html: LINK_SVG, {link(LinkType::Website, link_)} }
                        }
                        if !project.skills.is_empty() {
                            p {
                                i { {project.skills.join(", ")} }
                            }
                        }
                    }
                }
            }
        }
    }
}
