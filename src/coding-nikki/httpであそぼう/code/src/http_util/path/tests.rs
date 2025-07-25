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
