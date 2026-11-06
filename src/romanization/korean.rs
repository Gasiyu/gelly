pub(super) fn romanize(text: &str) -> Option<String> {
    Some(korean_romanize::convert(text))
}

pub(super) fn is_match(text: &str) -> bool {
    korean_romanize::has_korean(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanize_korean_text() {
        assert_eq!(romanize("안녕"), Some("annyeong".to_string()));
    }

    #[test]
    fn check_matches() {
        assert!(is_match("안녕"));
        assert!(is_match("안녕 Hello"));
        assert!(!is_match("Hello"));
    }
}
