use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpPath<'a>(&'a str);

impl<'a> HttpPath<'a> {
    // 許可された文字列のみで作る
    pub fn from_str(s: &'a str) -> Option<Self> {
        // 文字単位に分解します
        let mut c = s.chars();

        // 先頭は/になると見込んで
        if c.next() != Some('/') {
            return None;
        }

        // findメソッドで許可されていない文字があるか検索しましょう
        // なかったら成功です。
        if c.find(|c| {
            !(c.is_ascii_alphanumeric()
                || *c == '/'
                || *c == '-'
                || *c == '_'
                || *c == '.'
                || *c == '='
                || *c == '?'
                || *c == '&'
                || *c == '%'
                || *c == '#')
        }) == None
        {
            Some(HttpPath(s))
        } else {
            None
        }
    }
}

// Fromトレイトもつけて、文字列から簡単に変換できるようにしましょう
impl<'a> From<&'a str> for HttpPath<'a> {
    fn from(s: &'a str) -> Self {
        HttpPath::from_str(s).unwrap_or(HttpPath("/")) // デフォルト値は /
    }
}
// 文字列で取得できるように、Displayを実装しておきましょう
impl<'a> fmt::Display for HttpPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_http_path_from_str() {
        assert_eq!(
            HttpPath::from_str("/path/to/resource"),
            Some(HttpPath("/path/to/resource"))
        );
        assert_eq!(
            HttpPath::from_str("/path/to/resource?query=1"),
            Some(HttpPath("/path/to/resource?query=1"))
        );
        assert_eq!(HttpPath::from_str("/"), Some(HttpPath("/")));
    }

    #[test]
    fn failure_http_path_from_str() {
        assert_eq!(HttpPath::from_str(""), None);
        assert_eq!(HttpPath::from_str("/path/to/resource/*"), None);
        assert_eq!(HttpPath::from_str("path/to/resource"), None);
    }
}
