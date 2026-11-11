/// Romanizes Hangul text, returning `None` if none is detected or conversion leaves it unchanged.
pub(super) fn romanize(text: &str) -> Option<String> {
    korean_romanize::has_korean(text)
        .then(|| korean_romanize::convert(text))
        .filter(|romanized| romanized != text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanize_korean_text() {
        assert_eq!(romanize("안녕"), Some("annyeong".to_string()));
    }

    #[test]
    fn romanize_mixed_text() {
        assert_eq!(
            romanize("Hello, 안녕, こんにちは, 你好!"),
            Some("Hello, annyeong, こんにちは, 你好!".to_string())
        );
    }

    #[test]
    fn returns_none_without_korean_text() {
        assert_eq!(romanize("Hello"), None);
    }

    #[test]
    fn returns_none_for_empty_text() {
        assert_eq!(romanize(""), None);
    }

    #[test]
    fn returns_none_when_detected_hangul_is_unchanged_by_conversion() {
        assert_eq!(romanize("\u{D7A4}"), None);
    }
}
