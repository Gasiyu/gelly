//! Applies configured romanizers to text.
//!
//! Romanizers run in order, each receiving the previous successful output. A `None`
//! result leaves the current text unchanged.

mod korean;

type Romanizer = fn(&str) -> Option<String>;

const ROMANIZERS: &[Romanizer] = &[korean::romanize];

/// Romanizes a line, returning `None` if it is unchanged.
pub(crate) fn romanize_line(text: &str) -> Option<String> {
    romanize_with(text, ROMANIZERS)
}

fn romanize_with(text: &str, romanizers: &[Romanizer]) -> Option<String> {
    let mut romanized = None;

    for romanizer in romanizers {
        let input = romanized.as_deref().unwrap_or(text);
        if let Some(result) = romanizer(input) {
            romanized = Some(result);
        }
    }

    romanized.filter(|result| result != text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanize_line_returns_none_for_unsupported_text() {
        assert_eq!(romanize_line("Hello 123!"), None);
    }
}
