use super::*;

#[test]
fn success_response() {
    let response = r#"HTTP/1.1 200 OK"#;
    let response = HttpResponse::from_str(response).unwrap();
    assert_eq!(response.version, HttpVersion::Http11);
    assert_eq!(response.status, (200, "OK"));
    assert!(response.header.is_empty());
    assert!(response.body.is_empty());
}

#[test]
fn failure_response() {
    let response = HttpResponse::from_str("");
    assert!(response.is_none());
}
