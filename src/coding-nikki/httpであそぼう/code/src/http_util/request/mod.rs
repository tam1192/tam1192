use super::{utils::*, *};
use std::{collections::HashMap, fmt};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub path: HttpPath<'a>,
    pub version: HttpVersion,
    pub header: HashMap<&'a str, &'a str>,
    pub body: String,
}
impl<'a> HttpRequest<'a> {
    pub fn new(
        method: HttpMethod,
        path: HttpPath<'a>,
        version: HttpVersion,
        header: HashMap<&'a str, &'a str>,
        body: String,
    ) -> Self {
        Self {
            method,
            path,
            version,
            header,
            body,
        }
    }
    pub fn from_str(s: &'a str) -> Option<Self> {
        // 行取得で行う
        let mut lines = s.lines();

        // 1行目を取得する
        let mut parts = {
            let line = lines.next().unwrap_or("");
            line.split_whitespace() // スペース単位で分割させる
        };
        let method = HttpMethod::from_str(parts.next().unwrap_or(""))?;
        let path = HttpPath::from_str(parts.next().unwrap_or(""))?;
        let version = HttpVersion::from_str(parts.next().unwrap_or(""))?;
        // 余分にあったら無効とする
        if parts.next().is_some() {
            return None;
        }

        // 2行目(以降)を処理する
        let mut header: HashMap<&str, &str> = HashMap::new();
        loop {
            let line = lines.next().unwrap_or("");
            match line_parse_http_header(line) {
                Some((k, v)) => {
                    _ = header.insert(k, v);
                }
                None => break,
            }
        }

        // headerの処理をする
        let body = lines.collect::<String>();

        Some(Self {
            method,
            path,
            version,
            header,
            body,
        })
    }
}

// 文字列で取得できるように、Displayを実装しておきましょう
impl<'a> fmt::Display for HttpRequest<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\r\n", self.method, self.path, self.version)?;
        for (k, v) in &self.header {
            write!(f, "{}: {}\r\n", k, v)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}
