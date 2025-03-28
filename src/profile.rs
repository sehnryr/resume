use crate::element::*;
use crate::link::{
    LinkType,
    link,
};

const MAP_PIN_SVG: &str = include_str!("../assets/icons/map-pin.svg");
const MAIL_SVG: &str = include_str!("../assets/icons/mail.svg");
const PHONE_SVG: &str = include_str!("../assets/icons/phone.svg");
const GLOBE_SVG: &str = include_str!("../assets/icons/globe.svg");
const BLUESKY_SVG: &str = include_str!("../assets/icons/bluesky.svg");
const LINKEDIN_SVG: &str = include_str!("../assets/icons/linkedin.svg");

pub fn profile() -> String {
    div!(
        @class = "profile",
        img!(@src = "assets/images/profile.webp"),
        h1!("Youn Mélois"),
        contact(),
        language(),
        soft_skills(),
        hard_skills(),
        interests(),
    )
}

fn contact() -> String {
    let address: &str = "Nantes, France";

    let email: &str = "youn@melois.dev";
    let phone: &str = "+33 7 83 54 37 48";
    let website: &str = "melois.dev";
    let bluesky: &str = "melois.dev";
    let linkedin: &str = "youn-melois";

    section!(
        @class = "contact",
        ul!(
            li!(MAP_PIN_SVG, address),
            li!(MAIL_SVG, link(LinkType::Email, email)),
            li!(PHONE_SVG, link(LinkType::Phone, phone)),
            li!(GLOBE_SVG, link(LinkType::Website, website)),
            li!(BLUESKY_SVG, link(LinkType::Bluesky, bluesky)),
            li!(LINKEDIN_SVG, link(LinkType::Linkedin, linkedin)),
        )
    )
}

struct Language<'a> {
    name: &'a str,
    level: &'a str,
}

fn language() -> String {
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

    section!(
        @class = "language",
        h2!("Langues"),
        ul!(
            languages.iter().map(|language| {
                li!(
                    language.name,
                    span!(language.level)
                )
            })
        )
    )
}

fn soft_skills() -> String {
    let soft_skills = vec!["Vif d'esprit", "Ambitieux", "Prudent", "Ordonné"];

    section!(
        @class = "soft-skill",
        h2!("Compétences Humaines"),
        ul!(
            soft_skills.iter().map(|skill| {
                li!(skill)
            })
        )
    )
}

struct HardSkill<'a> {
    name: &'a str,
    examples: Vec<&'a str>,
}

fn hard_skills() -> String {
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

    section!(
        @class = "hard-skill",
        h2!("Compétences"),
        ul!(
            hard_skills.iter().map(|skill| {
                li!(
                    skill.name,
                    span!(skill.examples.join(", "))
                )
            })
        )
    )
}

struct Interest<'a> {
    name: &'a str,
    examples: Vec<&'a str>,
}

fn interests() -> String {
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

    section!(
        @class = "interest",
        h2!("Centres d'intérêt"),
        ul!(
            interests.iter().map(|interest| {
                li!(
                    interest.name,
                    span!(interest.examples.join(", "))
                )
            })
        )
    )
}
