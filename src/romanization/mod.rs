mod korean;

pub(crate) fn romanize_line(text: &str) -> Option<String> {
    let romanized = if korean::is_match(text) {
        korean::romanize(text)
    } else {
        None
    }?;

    (romanized != text).then_some(romanized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_mixed_and_unsupported_lines() {
        assert_eq!(
            romanize_line("지루하게 기다렸던 summer vacation"),
            Some("jiruhage gidaryeossdeon summer vacation".to_string())
        );
        assert_eq!(romanize_line("Hello 123!"), None);
    }
}
