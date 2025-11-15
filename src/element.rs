use core::fmt;

pub enum Element {
    Text(String),
    Children(Vec<Element>),
    Tag(String, Vec<(String, String)>, Vec<Element>),
}

impl Element {
    pub fn new(
        name: &str,
        attributes: Vec<(String, String)>,
        children: Vec<Element>,
    ) -> Element {
        Element::Tag(name.to_string(), attributes, children)
    }
}

impl fmt::Display for Element {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Element::Text(text) => write!(f, "{}", text),
            Element::Children(elements) => {
                for element in elements {
                    write!(f, "{}", element)?;
                }
                Ok(())
            }
            Element::Tag(name, attributes, children) => {
                write!(
                    f,
                    "<{}{}>",
                    name,
                    attributes
                        .iter()
                        .map(|(k, v)| format!(" {}=\"{}\"", k, v))
                        .collect::<Vec<String>>()
                        .join("")
                )?;
                for element in children {
                    write!(f, "{}", element)?;
                }
                write!(f, "</{}>", name)
            }
        }
    }
}

pub trait ToElement {
    fn to_element(self) -> Element;
}

impl<S: ToString> ToElement for S {
    fn to_element(self) -> Element { Element::Text(self.to_string()) }
}

pub trait ToElementArray {
    fn to_element(self) -> Element;
}

impl<I: Iterator<Item = impl ToElement>> ToElementArray for I {
    fn to_element(self) -> Element { Element::Children(self.map(ToElement::to_element).collect()) }
}

pub trait ToElementOption {
    fn to_element(self) -> Element;
}

impl<T: ToElement> ToElementOption for Option<T> {
    fn to_element(self) -> Element { ToElementArray::to_element(self.into_iter()) }
}

macro_rules! element {
    ($tag:ident) => {
        crate::element::Element::new(
            stringify!($tag),
            vec![],
            vec![],
        )
    };
    ($tag:ident $(,@ $key:pat = $value:expr)+ $(,)?) => {
        crate::element::Element::new(
            stringify!($tag),
            vec![$((stringify!($key).to_string(), $value.to_string())),+],
            vec![],
        )
    };
    ($tag:ident $(,$children:expr)+ $(,)?) => {
        crate::element::Element::new(
            stringify!($tag),
            vec![],
            vec![$($children.to_element()),+],
        )
    };
    ($tag:ident $(,@ $key:pat = $value:expr)+ $(,$children:expr)+ $(,)?) => {
        crate::element::Element::new(
            stringify!($tag),
            vec![$((stringify!($key).to_string(), $value.to_string())),+],
            vec![$($children.to_element()),+],
        )
    };
}

macro_rules! elements {
    (($d:tt) $($tag:ident),+) => {
        $(
            macro_rules! $tag {
                () => {
                    element!($tag)
                };
                ($d(@ $key:pat = $value:expr),+ $d(,)?) => {
                    element!($tag $d(,@ $key = $value)+)
                };
                ($d($children:expr),+ $d(,)?) => {
                    element!($tag $d(,$children)+)
                };
                ($d(@ $key:pat = $value:expr),+ $d(,$children:expr)+ $d(,)?) => {
                    element!($tag $d(,@ $key = $value)+ $d(,$children)+)
                };
            }
            pub(crate) use $tag;
        )+
    };
    ($($tag:ident),+) => {
        elements!{($) $($tag),+}
    };
}

elements!(
    html, meta, head, title, style, body, div, main, section, h1, h2, ul, li, p, span, img, a, i
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() {
        let v = div!();
        assert_eq!(v.to_string(), "<div></div>");

        let v = div!(@id = "");
        assert_eq!(v.to_string(), "<div id=\"\"></div>");

        let v = div!(@id = "test");
        assert_eq!(v.to_string(), "<div id=\"test\"></div>");

        let v = div!(@id = "test", @class = "container");
        assert_eq!(v.to_string(), "<div id=\"test\" class=\"container\"></div>");

        let v = div!("content");
        assert_eq!(v.to_string(), "<div>content</div>");

        let v = div!(@id = "test", @class = "container", "content");
        assert_eq!(
            v.to_string(),
            "<div id=\"test\" class=\"container\">content</div>"
        );

        let v = div!(
            @id = "test",
            @class = "container",
            "content",
            p!()
        );
        assert_eq!(
            v.to_string(),
            "<div id=\"test\" class=\"container\">content<p></p></div>"
        );

        let v = div!(
            @id = "test",
            @class = "container",
            "content",
            p!(@id = "p", "text")
        );
        assert_eq!(
            v.to_string(),
            "<div id=\"test\" class=\"container\">content<p id=\"p\">text</p></div>"
        );
    }
}
