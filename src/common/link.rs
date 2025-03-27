use dioxus::prelude::*;

pub enum LinkType {
    Email,
    Phone,
    Website,
    Bluesky,
    Linkedin,
}

pub fn link<S>(
    r#type: LinkType,
    value: S,
) -> Element
where
    S: AsRef<str>,
{
    let value = value.as_ref();

    let (href, text) = match r#type {
        LinkType::Email => (format!("mailto:{}", value), value.to_owned()),
        LinkType::Phone => (format!("tel:{}", value.replace(" ", "")), value.to_owned()),
        LinkType::Website => (format!("https://{}", value), format!("https://{}", value)),
        LinkType::Bluesky => (
            format!("https://bsky.app/profile/{}", value),
            format!("@{}", value),
        ),
        LinkType::Linkedin => (
            format!("https://linkedin.com/in/{}", value),
            format!("in/{}", value),
        ),
    };

    rsx! {
        a { href, target: "_blank", rel: "noopener noreferrer", "{text}" }
    }
}
