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
