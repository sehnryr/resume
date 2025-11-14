use crate::element::*;
use crate::link::{
    LinkType,
    link,
};
use crate::{
    Config,
    Contact,
    HardSkill,
    Interest,
    Language,
    SoftSkill,
};

const MAP_PIN_SVG: &str = include_str!("../assets/icons/map-pin.svg");
const MAIL_SVG: &str = include_str!("../assets/icons/mail.svg");
const PHONE_SVG: &str = include_str!("../assets/icons/phone.svg");
const GLOBE_SVG: &str = include_str!("../assets/icons/globe.svg");
const BLUESKY_SVG: &str = include_str!("../assets/icons/bluesky.svg");
const LINKEDIN_SVG: &str = include_str!("../assets/icons/linkedin.svg");

pub fn profile(
    config: &Config,
    lang: &str,
) -> Element {
    div!(
        @class = "profile",
        img!(
            @src = "/assets/images/profile.webp",
            @alt = match lang {
                "fr" => "Photo de profil",
                _ => "Profile picture",
            },
        ),
        h1!(&config.name),
        contact(&config.address, &config.contact),
        language(&config.languages, lang),
        soft_skills(&config.soft_skills, lang),
        hard_skills(&config.hard_skills, lang),
        interests(&config.interests, lang),
    )
}

fn contact(
    address: &str,
    contact: &Contact,
) -> Element {
    section!(
        @class = "contact",
        ul!(
            li!(MAP_PIN_SVG, &address),
            li!(MAIL_SVG, link(LinkType::Email, &contact.email)),
            li!(PHONE_SVG, link(LinkType::Phone, &contact.phone)),
            li!(GLOBE_SVG, link(LinkType::Website, &contact.website)),
            li!(BLUESKY_SVG, link(LinkType::Bluesky, &contact.bluesky)),
            li!(LINKEDIN_SVG, link(LinkType::Linkedin, &contact.linkedin)),
        )
    )
}

fn language(
    languages: &[Language],
    lang: &str,
) -> Element {
    section!(
        @class = "language",
        h2!(match lang {
            "fr" => "Langues",
            _ => "Languages",
        }),
        ul!(
            languages.iter().map(|language| {
                li!(
                    &language.name,
                    span!(&language.level)
                )
            })
        )
    )
}

fn soft_skills(
    soft_skills: &[SoftSkill],
    lang: &str,
) -> Element {
    section!(
        @class = "soft-skill",
        h2!(match lang {
            "fr" => "Compétences Humaines",
            _ => "Soft Skills",
        }),
        ul!(
            soft_skills.iter().map(|skill| {
                li!(&skill.name)
            })
        )
    )
}

fn hard_skills(
    hard_skills: &[HardSkill],
    lang: &str,
) -> Element {
    section!(
        @class = "hard-skill",
        h2!(match lang {
            "fr" => "Compétences",
            _ => "Hard Skills",
        }),
        ul!(
            hard_skills.iter().map(|skill| {
                li!(
                    &skill.name,
                    span!(skill.examples.join(", "))
                )
            })
        )
    )
}

fn interests(
    interests: &[Interest],
    lang: &str,
) -> Element {
    section!(
        @class = "interest",
        h2!(match lang {
            "fr" => "Centres d'intérêt",
            _ => "Interests",
        }),
        ul!(
            interests.iter().map(|interest| {
                li!(
                    &interest.name,
                    span!(interest.examples.join(", "))
                )
            })
        )
    )
}
