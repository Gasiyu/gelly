use pinyin_pro::{
    options::{PinyinOptions, PinyinOutput, TypeMode},
    pinyin,
};

/// Romanizes Chinese text to Hanyu Pinyin, returning `None` if no Chinese characters
/// are detected or if the conversion leaves the text unchanged.
///
/// Consecutive Chinese syllables are separated by spaces in the output.
pub(super) fn romanize(text: &str) -> Option<String> {
    let PinyinOutput::All(words) = pinyin(
        text,
        PinyinOptions {
            type_mode: TypeMode::All,
            traditional: true,
            ..Default::default()
        },
    ) else {
        return None;
    };

    let mut romanized = String::new();
    let mut previous_was_chinese = false;

    for word in words {
        let is_chinese = word.is_zh;
        if is_chinese && previous_was_chinese {
            romanized.push(' ');
        }

        let output = if is_chinese {
            &word.result
        } else {
            &word.origin
        };
        romanized.push_str(output);
        previous_was_chinese = is_chinese;
    }

    (romanized != text).then_some(romanized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn romanize_should_convert_simplified_chinese() {
        assert_eq!(romanize("你好"), Some("nǐ hǎo".to_string()));
    }

    #[test]
    fn romanize_should_convert_traditional_chinese() {
        assert_eq!(romanize("繁體中文"), Some("fán tǐ zhōng wén".to_string()));
    }

    #[test]
    fn romanize_should_preserve_non_chinese_in_mixed_text() {
        assert_eq!(
            romanize("Hello, 안녕, こんにちは, 你好!"),
            Some("Hello, 안녕, こんにちは, nǐ hǎo!".to_string())
        );
    }

    #[test]
    fn romanize_should_return_none_for_non_chinese_text() {
        assert_eq!(romanize("Hello"), None);
    }

    #[test]
    fn romanize_should_return_none_for_empty_text() {
        assert_eq!(romanize(""), None);
    }
}
