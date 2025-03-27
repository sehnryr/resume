use dioxus::prelude::*;

use crate::common::{
    LinkType,
    link,
};

const MAP_PIN_SVG: &str = include_str!("../assets/icons/map-pin.svg");
const MAIL_SVG: &str = include_str!("../assets/icons/mail.svg");
const PHONE_SVG: &str = include_str!("../assets/icons/phone.svg");
const GLOBE_SVG: &str = include_str!("../assets/icons/globe.svg");
const BLUESKY_SVG: &str = include_str!("../assets/icons/bluesky.svg");
const LINKEDIN_SVG: &str = include_str!("../assets/icons/linkedin.svg");

pub fn profile() -> Element {
    rsx! {
        div { class: "profile",
            img { src: "assets/images/profile.webp" }
            h1 { "Youn Mélois" }
            {contact()}
            {language()}
            {soft_skills()}
            {hard_skills()}
            {interests()}
        }
    }
}

fn contact() -> Element {
    let address: &str = "Nantes, France";

    let email: &str = "youn@melois.dev";
    let phone: &str = "+33 7 83 54 37 48";
    let website: &str = "melois.dev";
    let bluesky: &str = "melois.dev";
    let linkedin: &str = "youn-melois";

    rsx! {
        section { class: "contact",
            ul {
                li { dangerous_inner_html: MAP_PIN_SVG, {address} }
                li { dangerous_inner_html: MAIL_SVG, {link(LinkType::Email, email)} }
                li { dangerous_inner_html: PHONE_SVG, {link(LinkType::Phone, phone)} }
                li { dangerous_inner_html: GLOBE_SVG, {link(LinkType::Website, website)} }
                li { dangerous_inner_html: BLUESKY_SVG, {link(LinkType::Bluesky, bluesky)} }
                li { dangerous_inner_html: LINKEDIN_SVG, {link(LinkType::Linkedin, linkedin)} }
            }
        }
    }
}

struct Language<'a> {
    name: &'a str,
    level: &'a str,
}

fn language() -> Element {
    let languages = vec![
        Language {
            name: "Français",
            level: "Natif",
        },
        Language {
            name: "Anglais (C1)",
            level: "Professionnel",
        },
    ];

    rsx! {
        section { class: "language",
            h2 { "Langues" }
            ul {
                for language in &languages {
                    li {
                        {language.name}
                        span { {language.level} }
                    }
                }
            }
        }
    }
}

fn soft_skills() -> Element {
    let soft_skills = vec!["Vif d'esprit", "Ambitieux", "Prudent", "Ordonné"];

    rsx! {
        section { class: "soft-skill",
            h2 { "Compétences Humaines" }
            ul {
                for skill in &soft_skills {
                    li { {skill} }
                }
            }
        }
    }
}

struct HardSkill<'a> {
    name: &'a str,
    examples: Vec<&'a str>,
}

fn hard_skills() -> Element {
    let hard_skills = vec![
        HardSkill {
            name: "Rust",
            examples: vec!["CLI", "FFI", "WebAssembly"],
        },
        HardSkill {
            name: "Opérationnel",
            examples: vec!["Ansible", "Docker", "OpenStack"],
        },
        HardSkill {
            name: "C/C++",
            examples: vec!["IHM", "Qt"],
        },
        HardSkill {
            name: "Linux",
            examples: vec!["Arch", "Bash"],
        },
    ];

    rsx! {
        section { class: "hard-skill",
            h2 { "Compétences" }
            ul {
                for skill in &hard_skills {
                    li {
                        {skill.name}
                        span { {skill.examples.join(", ")} }
                    }
                }
            }
        }
    }
}

struct Interest<'a> {
    name: &'a str,
    examples: Vec<&'a str>,
}

fn interests() -> Element {
    let interests = vec![
        Interest {
            name: "Informatique",
            examples: vec!["Open-source"],
        },
        Interest {
            name: "Sport",
            examples: vec!["Vélo"],
        },
        Interest {
            name: "Lecture",
            examples: vec!["Comic", "Webtoon"],
        },
        Interest {
            name: "Jeux vidéo",
            examples: vec!["Bac à Sable", "Coop"],
        },
    ];

    rsx! {
        section { class: "interest",
            h2 { "Centres d'intérêt" }
            ul {
                for interest in &interests {
                    li {
                        {interest.name}
                        span { {interest.examples.join(", ")} }
                    }
                }
            }
        }
    }
}
