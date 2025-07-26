#[cfg(test)]
pub mod tests_line_parse_http_header {
    use super::super::*;

    #[test]
    fn success_curl_header() {
        assert_eq!(
            line_parse_http_header("Host: localhost").unwrap(),
            ("Host", "localhost")
        );
        assert_eq!(
            line_parse_http_header("User-Agent: curl/7.64.1").unwrap(),
            ("User-Agent", "curl/7.64.1")
        );
        assert_eq!(
            line_parse_http_header("Accept: */*").unwrap(),
            ("Accept", "*/*")
        );
    }

    #[test]
    fn success_safari_header() {
        assert_eq!(
            line_parse_http_header("Host: 127.0.0.1").unwrap(),
            ("Host", "127.0.0.1")
        );
        assert_eq!(
            line_parse_http_header("Sec-Fetch-Dest: document").unwrap(),
            ("Sec-Fetch-Dest", "document")
        );
        assert_eq!(
            line_parse_http_header("User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15").unwrap(),
            (
                "User-Agent",
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/18.5 Safari/605.1.15"
            )
        );
        assert_eq!(
            line_parse_http_header("Upgrade-Insecure-Requests: 1").unwrap(),
            ("Upgrade-Insecure-Requests", "1")
        );
        assert_eq!(
            line_parse_http_header(
                "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
            )
            .unwrap(),
            (
                "Accept",
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
            )
        );
        assert_eq!(
            line_parse_http_header("Sec-Fetch-Site: none").unwrap(),
            ("Sec-Fetch-Site", "none")
        );
        assert_eq!(
            line_parse_http_header("Sec-Fetch-Mode: navigate").unwrap(),
            ("Sec-Fetch-Mode", "navigate")
        );
        assert_eq!(
            line_parse_http_header("Accept-Language: ja").unwrap(),
            ("Accept-Language", "ja")
        );
        assert_eq!(
            line_parse_http_header("Priority: u=0, i").unwrap(),
            ("Priority", "u=0, i")
        );
        assert_eq!(
            line_parse_http_header("Accept-Encoding: gzip, deflate").unwrap(),
            ("Accept-Encoding", "gzip, deflate")
        );
        assert_eq!(
            line_parse_http_header("Connection: keep-alive").unwrap(),
            ("Connection", "keep-alive")
        );
    }

    #[test]
    fn success_other_header() {
        assert_eq!(
            line_parse_http_header("Content-Type: application/json").unwrap(),
            ("Content-Type", "application/json")
        );
        assert_eq!(
            line_parse_http_header("Authorization: Bearer token123").unwrap(),
            ("Authorization", "Bearer token123")
        );
        assert_eq!(
            line_parse_http_header("Cache-Control: no-cache").unwrap(),
            ("Cache-Control", "no-cache")
        );
        assert_eq!(
            line_parse_http_header("Expires: Wed, 21 Oct 2015 07:28:00 GMT").unwrap(),
            ("Expires", "Wed, 21 Oct 2015 07:28:00 GMT")
        );
    }

    #[test]
    fn failure_test() {
        assert_eq!(line_parse_http_header(""), None);
        assert_eq!(line_parse_http_header("NoColonHeader"), None);
    }
}
