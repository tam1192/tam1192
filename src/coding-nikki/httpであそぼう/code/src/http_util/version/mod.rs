use std::fmt;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpVersion {
    Http10,
    Http11,
    Http20,
    Http30,
}
impl HttpVersion {
    pub fn from_str(s: &str) -> Option<Self> {
        // 大文字/小文字を考慮しない。(小文字に統一)
        match s.to_lowercase().as_str() {
            "http/1.0" => Some(HttpVersion::Http10),
            "http/1.1" => Some(HttpVersion::Http11),
            "http/2.0" => Some(HttpVersion::Http20),
            "http/3.0" => Some(HttpVersion::Http30),
            _ => None,
        }
    }
}
impl From<&str> for HttpVersion {
    fn from(s: &str) -> Self {
        HttpVersion::from_str(s).unwrap_or(HttpVersion::Http10)
    }
}
// 文字列で取得できるように、Displayを実装しておきましょう
impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Http10 => "HTTP/1.0",
                Self::Http11 => "HTTP/1.1",
                Self::Http20 => "HTTP/2.0",
                Self::Http30 => "HTTP/3.0",
            }
        )
    }
}
