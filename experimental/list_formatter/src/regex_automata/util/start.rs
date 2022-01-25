#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Start {
    NonWordByte = 0,
    WordByte = 1,
    Text = 2,
    Line = 3,
}

impl Start {
    pub(crate) fn from_usize(n: usize) -> Option<Start> {
        match n {
            0 => Some(Start::NonWordByte),
            1 => Some(Start::WordByte),
            2 => Some(Start::Text),
            3 => Some(Start::Line),
            _ => None,
        }
    }

    pub(crate) fn count() -> usize {
        4
    }

    #[inline(always)]
    pub(crate) fn from_position_fwd(bytes: &[u8], start: usize, end: usize) -> Start {
        assert!(
            bytes.get(start..end).is_some(),
            "{}..{} is invalid",
            start,
            end
        );
        if start == 0 {
            Start::Text
        } else if bytes[start - 1] == b'\n' {
            Start::Line
        } else if crate::regex_automata::util::is_word_byte(bytes[start - 1]) {
            Start::WordByte
        } else {
            Start::NonWordByte
        }
    }

    #[inline(always)]
    pub(crate) fn from_position_rev(bytes: &[u8], start: usize, end: usize) -> Start {
        assert!(
            bytes.get(start..end).is_some(),
            "{}..{} is invalid",
            start,
            end
        );
        if end == bytes.len() {
            Start::Text
        } else if bytes[end] == b'\n' {
            Start::Line
        } else if crate::regex_automata::util::is_word_byte(bytes[end]) {
            Start::WordByte
        } else {
            Start::NonWordByte
        }
    }

    #[inline(always)]
    pub(crate) fn as_usize(&self) -> usize {
        *self as usize
    }
}
