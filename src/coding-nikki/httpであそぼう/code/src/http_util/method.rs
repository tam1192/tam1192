use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}
impl HttpMethod {
    pub fn from_str(s: &str) -> Option<Self> {
        // 大文字/小文字を考慮しない。(小文字に統一)
        match s.to_lowercase().as_str() {
            "get" => Some(HttpMethod::Get),
            "post" => Some(HttpMethod::Post),
            "put" => Some(HttpMethod::Put),
            "delete" => Some(HttpMethod::Delete),
            _ => None,
        }
    }
}
impl From<&str> for HttpMethod {
    fn from(s: &str) -> Self {
        HttpMethod::from_str(s).unwrap_or(HttpMethod::Get)
    }
}
// 文字列で取得できるように、Displayを実装しておきましょう
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Get => "GET",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Delete => "DELETE",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success_from_str() {
        assert_eq!(
            super::HttpMethod::from_str("GET"),
            Some(super::HttpMethod::Get)
        );
        assert_eq!(
            super::HttpMethod::from_str("POST"),
            Some(super::HttpMethod::Post)
        );
        assert_eq!(
            super::HttpMethod::from_str("PUT"),
            Some(super::HttpMethod::Put)
        );
        assert_eq!(
            super::HttpMethod::from_str("DELETE"),
            Some(super::HttpMethod::Delete)
        );
        assert_eq!(
            super::HttpMethod::from_str("get"),
            Some(super::HttpMethod::Get)
        );
        assert_eq!(
            super::HttpMethod::from_str("post"),
            Some(super::HttpMethod::Post)
        );
        assert_eq!(
            super::HttpMethod::from_str("put"),
            Some(super::HttpMethod::Put)
        );
        assert_eq!(
            super::HttpMethod::from_str("delete"),
            Some(super::HttpMethod::Delete)
        );
    }

    #[test]
    fn failure_from_str() {
        assert!(super::HttpMethod::from_str("").is_none());
        assert!(super::HttpMethod::from_str("hello").is_none());
        assert!(super::HttpMethod::from_str("HELLO").is_none());
    }
}
