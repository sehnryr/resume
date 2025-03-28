pub trait IntoString {
    fn into_string(self) -> String;
}

pub trait IntoStringIterator {
    type Item: IntoString;
    fn into_string(self) -> String;
}

pub trait IntoStringOption {
    fn into_string(self) -> String;
}

impl<S: ToString> IntoString for S {
    fn into_string(self) -> String { self.to_string() }
}

impl<I: Iterator<Item = impl IntoString>> IntoStringIterator for I {
    type Item = I::Item;

    fn into_string(self) -> String {
        self.map(IntoString::into_string)
            .collect::<Vec<String>>()
            .join("")
    }
}

impl<T: IntoString> IntoStringOption for Option<T> {
    fn into_string(self) -> String { self.map(IntoString::into_string).unwrap_or_default() }
}

macro_rules! elements {
    (($d:tt) $($tag:ident),*) => {
        $(
            macro_rules! $tag {
                (@attributes $d(@$key:pat = $value:expr),*) => {
                    (vec![$d(format!(" {}=\"{}\"", stringify!($key), $value)),*] as Vec<String>).join("")
                };
                (@contents $d($content:expr),*) => {
                    (vec![$d($content.into_string()),*] as Vec<String>).join("")
                };
                ($d(@$key:pat = $value:expr),* $d(,)?) => {
                    format!(
                        "<{}{}></{}>",
                        stringify!($tag),
                        $tag!(@attributes $d(@$key = $value),*),
                        stringify!($tag)
                    )
                };
                ($d($content:expr),* $d(,)?) => {
                    format!(
                        "<{}>{}</{}>",
                        stringify!($tag),
                        $tag!(@contents $d($content),*),
                        stringify!($tag)
                    )
                };
                ($d(@$key:pat = $value:expr),+ , $d($content:expr),+ $d(,)?) => {
                    format!(
                        "<{}{}>{}</{}>",
                        stringify!($tag),
                        $tag!(@attributes $d(@$key = $value),*),
                        $tag!(@contents $d($content),*),
                        stringify!($tag)
                    )
                };
            }
            pub(crate) use $tag;
        )*
    };
    ($($tag:ident),*) => {
        elements!{($) $($tag),*}
    }
}

elements!(
    html, head, title, style, body, div, main, section, h1, h2, ul, li, p, span, img, a, i
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_element() {
        let v = div!();
        assert_eq!(v, "<div></div>");

        let v = div!(@id = "");
        assert_eq!(v, "<div id=\"\"></div>");

        let v = div!(@id = "test");
        assert_eq!(v, "<div id=\"test\"></div>");

        let v = div!(@id = "test", @class = "container");
        assert_eq!(v, "<div id=\"test\" class=\"container\"></div>");

        let v = div!("content");
        assert_eq!(v, "<div>content</div>");

        let v = div!(@id = "test", @class = "container", "content");
        assert_eq!(v, "<div id=\"test\" class=\"container\">content</div>");

        let v = div!(
            @id = "test",
            @class = "container",
            "content",
            p!()
        );
        assert_eq!(
            v,
            "<div id=\"test\" class=\"container\">content<p></p></div>"
        );

        let v = div!(
            @id = "test",
            @class = "container",
            "content",
            p!(@id = "p", "text")
        );
        assert_eq!(
            v,
            "<div id=\"test\" class=\"container\">content<p id=\"p\">text</p></div>"
        );
    }
}
