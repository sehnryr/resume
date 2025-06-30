use crate::element::*;
use crate::link::{
    LinkType,
    link,
};
use crate::{
    Config,
    Education,
    Experience,
    Project,
};

const LINK_SVG: &str = include_str!("../assets/icons/link-2.svg");

pub fn resume(
    config: &Config,
    lang: &str,
) -> Element {
    div!(
        @class = "resume",
        h1!(&config.title),
        h2!(&config.subtitle),
        experiences(&config.experiences, lang),
        projects(&config.projects, lang),
        education(&config.education, lang),
    )
}

fn experiences(
    experiences: &[Experience],
    lang: &str,
) -> Element {
    section!(
        @class = "experience",
        h2!(match lang {
            "fr" => "Expériences professionnelles",
            _ => "Professional Experiences",
        }),
        ul!(
            experiences.iter().map(|experience| {
                li!(
                    &experience.company,
                    span!(@class = "date", &experience.time ),
                    p!(&experience.title),
                    experience.description.as_ref().map(|description| {
                        p!(description)
                    }),
                    experience.link.as_ref().map(|link_| {
                        div!(LINK_SVG, link(LinkType::Website, link_))
                    }),
                    (!experience.skills.is_empty()).then(|| {
                        p!(i!(experience.skills.join(", ")))
                    })
                )
            })
        )
    )
}

fn education(
    education: &[Education],
    lang: &str,
) -> Element {
    section!(
        @class = "education",
        h2!(match lang {
            "fr" => "Formation",
            _ => "Education",
        }),
        ul!(
            education.iter().map(|education| {
                li!(
                    &education.school,
                    span!(@class = "date", &education.time),
                    p!(&education.degree),
                    education.description.as_ref().map(|description| {
                        p!(description)
                    }),
                    education.link.as_ref().map(|link_| {
                        div!(LINK_SVG, link(LinkType::Website, link_))
                    })
                )
            })
        )
    )
}

fn projects(
    projects: &[Project],
    lang: &str,
) -> Element {
    section!(
        @class = "project",
        h2!(match lang {
            "fr" => "Projets Personnels",
            _ => "Personal Projects",
        }),
        ul!(
            projects.iter().map(|project| {
                li!(
                    &project.title,
                    p!(&project.description),
                    project.link.as_ref().map(|link_| {
                        div!(LINK_SVG, link(LinkType::Website, link_))
                    }),
                    (!project.skills.is_empty()).then(|| {
                        p!(i!(project.skills.join(", ")))
                    })
                )
            })
        )
    )
}
