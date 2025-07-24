use super::*;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest<'a> {
    method: HttpMethod,
    path: HttpPath<'a>,
    version: HttpVersion,
    header: HashMap<&'a str, &'a str>,
    body: String,
}
impl<'a> HttpRequest<'a> {
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
            match utils::line_parse_http_header(line) {
                Some((k, v)) => {
                    _ = header.insert(k, v);
                }
                None => break,
            }
        }

        // headerの処理をする
        let body = lines.collect::<String>();

        Some(HttpRequest {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_curl_request() {
        let request = HttpRequest::from_str(
            "GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n",
        )
        .unwrap();
        assert_eq!(request.method, HttpMethod::Get);
        assert_eq!(request.path, HttpPath::from("/index.html"));
        assert_eq!(request.version, HttpVersion::Http11);
        assert_eq!(request.header.get("Host"), Some(&"example.com"));
        assert_eq!(request.header.get("User-Agent"), Some(&"curl/7.64.1"));
        assert_eq!(request.header.get("Accept"), Some(&"*/*"));
        assert_eq!(request.body, "");
    }

    #[test]
    fn success_safari_request() {
        let request_str = r#"GET / HTTP/1.1
Host: localhost
Sec-Fetch-Dest: document
User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15
Upgrade-Insecure-Requests: 1
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Sec-Fetch-Site: none
Sec-Fetch-Mode: navigate
Accept-Language: ja
Priority: u=0, i
Accept-Encoding: gzip, deflate
Connection: keep-alive
        
"#;
        let request = HttpRequest::from_str(request_str).unwrap();
        assert_eq!(request.method, HttpMethod::Get);
        assert_eq!(request.path, HttpPath::from("/"));
        assert_eq!(request.version, HttpVersion::Http11);
        assert_eq!(request.header.get("Host"), Some(&"localhost"));
        assert_eq!(
            request.header.get("User-Agent"),
            Some(
                &"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15"
            )
        );
        assert_eq!(
            request.header.get("Accept"),
            Some(&"text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
        );
        assert_eq!(request.body, "");
    }

    #[test]
    fn failure_request() {
        let request = HttpRequest::from_str("");
        assert!(request.is_none());
        let request = HttpRequest::from_str("GET");
        assert!(request.is_none());
        let request = HttpRequest::from_str("GET /");
        assert!(request.is_none());
        let request = HttpRequest::from_str("GET / HTTP/1.1");
        assert!(request.is_some());
    }
}
