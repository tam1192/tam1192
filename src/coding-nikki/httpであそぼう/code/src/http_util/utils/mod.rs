mod tests;

pub fn line_parse_http_header(s: &str) -> Option<(&str, &str)> {
    // 最初の:を探す
    let i = s.find(':')?;

    Some((&s[0..i], &s[i + 1..].trim())) // :でk/vを切り分けて終わり
}
