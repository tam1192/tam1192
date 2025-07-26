use super::*;

#[test]
fn success_http_path_from_str() {
    assert_eq!(
        HttpPath::from("/path/to/resource").to_string(),
        String::from("/path/to/resource")
    );
    assert_eq!(
        HttpPath::from("/path/to/resource?query=1").to_string(),
        String::from("/path/to/resource?query=1")
    );
    assert_eq!(HttpPath::from("/").to_string(), String::from("/"));
}

#[test]
fn failure_http_path_from_str() {
    assert_eq!(HttpPath::from_str(String::from("")), None);
    assert_eq!(
        HttpPath::from_str(String::from("/path/to/resource/*")),
        None
    );
    assert_eq!(HttpPath::from_str(String::from("path/to/resource")), None);
}
