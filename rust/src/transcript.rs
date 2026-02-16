/// Remove `<|...|>` markers from SenseVoice output and collapse whitespace.
pub fn clean_transcript(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let bytes = raw.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        // Try to match `<|...|>` pattern
        if bytes[i] == b'<' && i + 1 < bytes.len() && bytes[i + 1] == b'|' {
            // Search for closing `|>`
            if let Some(end) = raw[i + 2..].find("|>") {
                i = i + 2 + end + 2; // skip past `|>`
                continue;
            }
        }
        // Safe: we only enter here for valid UTF-8 char boundaries
        let c = raw[i..].chars().next().unwrap();
        out.push(c);
        i += c.len_utf8();
    }

    // collapse whitespace
    let mut result = String::with_capacity(out.len());
    let mut prev_ws = false;
    for c in out.chars() {
        if c.is_ascii_whitespace() {
            if !prev_ws {
                result.push(' ');
            }
            prev_ws = true;
        } else {
            result.push(c);
            prev_ws = false;
        }
    }

    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_language_and_emotion_markers() {
        let raw = "<|zh|><|NEUTRAL|><|Speech|><|woitn|>你好世界";
        assert_eq!(clean_transcript(raw), "你好世界");
    }

    #[test]
    fn removes_markers_with_surrounding_text() {
        let raw = "hello <|en|> world <|HAPPY|> test";
        assert_eq!(clean_transcript(raw), "hello world test");
    }

    #[test]
    fn collapses_whitespace() {
        let raw = "  hello   world  ";
        assert_eq!(clean_transcript(raw), "hello world");
    }

    #[test]
    fn handles_empty_string() {
        assert_eq!(clean_transcript(""), "");
    }

    #[test]
    fn handles_only_markers() {
        let raw = "<|zh|><|NEUTRAL|><|Speech|><|woitn|>";
        assert_eq!(clean_transcript(raw), "");
    }

    #[test]
    fn preserves_normal_angle_brackets() {
        let raw = "a < b and c > d";
        assert_eq!(clean_transcript(raw), "a < b and c > d");
    }

    #[test]
    fn handles_itn_markers() {
        let raw = "<|zh|><|NEUTRAL|><|Speech|><|withitn|>今天是2024年1月1日。";
        assert_eq!(clean_transcript(raw), "今天是2024年1月1日。");
    }
}
