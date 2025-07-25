use super::*;
use std::{collections::HashMap, fmt};
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse<'a> {
    version: HttpVersion,
    status: (u16, &'a str), // (ステータスコード, ステータスメッセージ)
    header: HashMap<&'a str, &'a str>,
    body: String,
}
impl<'a> HttpResponse<'a> {
    pub fn from_str(s: &'a str) -> Option<Self> {
        // 行取得で行う
        let mut lines = s.lines();

        // 1行目を取得する
        let mut parts = {
            let line = lines.next().unwrap_or("");
            line.split_whitespace() // スペース単位で分割させる
        };
        let version = HttpVersion::from_str(parts.next().unwrap_or(""))?;
        let status_code = parts.next().and_then(|s| s.parse::<u16>().ok())?;
        let status_message = parts.next().unwrap_or("");
        // 余分にあったら無効とする
        if parts.next().is_some() {
            return None;
        }

        // 2行目(以降)を処理する
        let mut header: HashMap<&str, &str> = HashMap::new();
        loop {
            let line = lines.next().unwrap_or("");
            match utils::line_parse_http_header(line) {
                Some((k, v)) => {
                    _ = header.insert(k, v);
                }
                None => break,
            }
        }

        // headerの処理をする
        let body = lines.collect::<String>();

        Some(Self {
            version,
            status: (status_code, status_message),
            header,
            body,
        })
    }
}

// 文字列で取得できるように、Displayを実装しておきましょう
impl<'a> fmt::Display for HttpResponse<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}\r\n",
            self.version, self.status.0, self.status.1
        )?;
        for (k, v) in &self.header {
            write!(f, "{}: {}\r\n", k, v)?;
        }
        write!(f, "\r\n{}", self.body)
    }
}
