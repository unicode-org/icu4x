use core::{ascii, fmt, str};

#[cfg(feature = "std")]
use alloc::vec::Vec;

pub mod alphabet;
pub(crate) mod bytes;
#[cfg(feature = "std")]
pub(crate) mod determinize;
pub mod id;
#[cfg(feature = "std")]
pub(crate) mod lazy;
pub(crate) mod matchtypes;
pub mod prefilter;
#[cfg(feature = "std")]
pub(crate) mod sparse_set;
pub(crate) mod start;
#[cfg(feature = "std")]
pub(crate) mod syntax;

pub(crate) const MATCH_OFFSET: usize = 1;

pub(crate) struct DebugByte(pub u8);

impl fmt::Debug for DebugByte {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 10 bytes is enough to cover any output from ascii::escape_default.
        let mut bytes = [0u8; 10];
        let mut len = 0;
        for (i, mut b) in ascii::escape_default(self.0).enumerate() {
            // capitalize \xab to \xAB
            if i >= 2 && b'a' <= b && b <= b'f' {
                b -= 32;
            }
            bytes[len] = b;
            len += 1;
        }
        write!(f, "{}", str::from_utf8(&bytes[..len]).unwrap())
    }
}

#[inline(always)]
pub(crate) fn next_utf8(text: &[u8], i: usize) -> usize {
    let b = match text.get(i) {
        None => return i.checked_add(1).unwrap(),
        Some(&b) => b,
    };
    // For cases where we see an invalid UTF-8 byte, there isn't much we can do
    // other than just start at the next byte.
    let inc = utf8_len(b).unwrap_or(1);
    i.checked_add(inc).unwrap()
}

#[inline(always)]
pub(crate) fn is_word_byte(b: u8) -> bool {
    match b {
        b'_' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}

#[inline(always)]
pub(crate) fn decode_utf8(bytes: &[u8]) -> Option<Result<char, u8>> {
    if bytes.is_empty() {
        return None;
    }
    let len = match utf8_len(bytes[0]) {
        None => return Some(Err(bytes[0])),
        Some(len) if len > bytes.len() => return Some(Err(bytes[0])),
        Some(1) => return Some(Ok(bytes[0] as char)),
        Some(len) => len,
    };
    match str::from_utf8(&bytes[..len]) {
        Ok(s) => Some(Ok(s.chars().next().unwrap())),
        Err(_) => Some(Err(bytes[0])),
    }
}

#[inline(always)]
pub(crate) fn decode_last_utf8(bytes: &[u8]) -> Option<Result<char, u8>> {
    if bytes.is_empty() {
        return None;
    }
    let mut start = bytes.len() - 1;
    let limit = bytes.len().saturating_sub(4);
    while start > limit && !is_leading_or_invalid_utf8_byte(bytes[start]) {
        start -= 1;
    }
    match decode_utf8(&bytes[start..]) {
        None => None,
        Some(Ok(ch)) => Some(Ok(ch)),
        Some(Err(_)) => Some(Err(bytes[bytes.len() - 1])),
    }
}

#[inline(always)]
fn utf8_len(byte: u8) -> Option<usize> {
    if byte <= 0x7F {
        return Some(1);
    } else if byte & 0b1100_0000 == 0b1000_0000 {
        return None;
    } else if byte <= 0b1101_1111 {
        Some(2)
    } else if byte <= 0b1110_1111 {
        Some(3)
    } else if byte <= 0b1111_0111 {
        Some(4)
    } else {
        None
    }
}

#[inline(always)]
fn is_leading_or_invalid_utf8_byte(b: u8) -> bool {
    // In the ASCII case, the most significant bit is never set. The leading
    // byte of a 2/3/4-byte sequence always has the top two most significant
    // bits set. For bytes that can never appear anywhere in valid UTF-8, this
    // also returns true, since every such byte has its two most significant
    // bits set:
    //
    //     \xC0 :: 11000000
    //     \xC1 :: 11000001
    //     \xF5 :: 11110101
    //     \xF6 :: 11110110
    //     \xF7 :: 11110111
    //     \xF8 :: 11111000
    //     \xF9 :: 11111001
    //     \xFA :: 11111010
    //     \xFB :: 11111011
    //     \xFC :: 11111100
    //     \xFD :: 11111101
    //     \xFE :: 11111110
    //     \xFF :: 11111111
    (b & 0b1100_0000) != 0b1000_0000
}

#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn is_word_char_fwd(bytes: &[u8], mut at: usize) -> bool {
    use core::{ptr, sync::atomic::AtomicPtr};

    use crate::regex_automata::{
        dfa::{
            dense::{self, DFA},
            Automaton,
        },
        util::lazy,
    };

    static WORD: AtomicPtr<DFA<Vec<u32>>> = AtomicPtr::new(ptr::null_mut());

    let dfa = lazy::get_or_init(&WORD, || {
        // TODO: Should we use a lazy DFA here instead? It does complicate
        // things somewhat, since we then need a mutable cache, which probably
        // means a thread local.
        dense::Builder::new()
            .configure(dense::Config::new().anchored(true))
            .build(r"\w")
            .unwrap()
    });
    // This is OK since '\w' contains no look-around.
    let mut sid = dfa.universal_start_state();
    while at < bytes.len() {
        let byte = bytes[at];
        sid = dfa.next_state(sid, byte);
        at += 1;
        if dfa.is_special_state(sid) {
            if dfa.is_match_state(sid) {
                return true;
            } else if dfa.is_dead_state(sid) {
                return false;
            }
        }
    }
    dfa.is_match_state(dfa.next_eoi_state(sid))
}

#[cfg(feature = "std")]
#[inline(always)]
pub(crate) fn is_word_char_rev(bytes: &[u8], mut at: usize) -> bool {
    use core::{ptr, sync::atomic::AtomicPtr};

    use crate::regex_automata::{
        dfa::{
            dense::{self, DFA},
            Automaton,
        },
        nfa::thompson::NFA,
    };

    static WORD: AtomicPtr<DFA<Vec<u32>>> = AtomicPtr::new(ptr::null_mut());

    let dfa = lazy::get_or_init(&WORD, || {
        dense::Builder::new()
            .configure(dense::Config::new().anchored(true))
            .thompson(NFA::config().reverse(true).shrink(true))
            .build(r"\w")
            .unwrap()
    });

    // This is OK since '\w' contains no look-around.
    let mut sid = dfa.universal_start_state();
    while at > 0 {
        at -= 1;
        let byte = bytes[at];
        sid = dfa.next_state(sid, byte);
        if dfa.is_special_state(sid) {
            if dfa.is_match_state(sid) {
                return true;
            } else if dfa.is_dead_state(sid) {
                return false;
            }
        }
    }
    dfa.is_match_state(dfa.next_eoi_state(sid))
}
