use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpPath(String);

impl HttpPath {
    // 許可された文字列のみで作る
    pub fn from_str(s: String) -> Option<Self> {
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
impl From<&str> for HttpPath {
    fn from(s: &str) -> Self {
        let s = s.to_string();
        HttpPath::from(s)
    }
}
impl From<String> for HttpPath {
    fn from(s: String) -> Self {
        HttpPath::from_str(s).unwrap_or(HttpPath(String::from("/"))) // デフォルト値は /
    }
}

// 文字列で取得できるように、Displayを実装しておきましょう
impl<'a> fmt::Display for HttpPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
