// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{impl_display_with_writeable, LengthHint, PartsWrite, Writeable};
use core::fmt;

/// A [`Writeable`] adapter that replaces occurrences of a needle with a replacement.
#[derive(Debug)]
#[allow(clippy::exhaustive_structs)] // designed for nesting
pub struct Replace<A, B, C> {
    /// The source writeable.
    pub source: A,
    /// The needle to search for.
    pub needle: B,
    /// The replacement writeable.
    pub replacement: C,
}

// Helper function to get the character at a specific index in a string slice.
// Since Rust strings are UTF-8, this is an O(N) operation, but it is acceptable
// because we assume the needle is small.
fn get_char(s: &str, index: usize) -> Option<char> {
    s.chars().nth(index)
}

// Computes the Knuth-Morris-Pratt (KMP) prefix function (failure function) value
// for the character at index `i` in `needle`.
//
// The prefix function value `pi[i]` is the length of the longest proper prefix
// of `needle[0..=i]` that is also a suffix of `needle[0..=i]`.
//
// This is computed on the fly without allocation.
fn get_pi_char(needle: &str, i: usize) -> usize {
    for k in (1..=i).rev() {
        let prefix = needle.chars().take(k);
        let suffix = needle.chars().take(i + 1).skip(i + 1 - k);
        if prefix.eq(suffix) {
            return k;
        }
    }
    0
}

// A writer wrapper that performs streaming replacement.
// It intercepts characters written to it, matches them against `needle` using KMP,
// and writes `replacement` when a full match is found, or the original characters otherwise.
struct ReplaceWriter<'a, W: ?Sized, B, C> {
    // The underlying sink to write to.
    sink: &'a mut W,
    // The needle we are searching for.
    needle: &'a B,
    // The replacement to write when the needle is matched.
    replacement: &'a C,
    // The number of characters of `needle` matched so far.
    matched_chars: usize,
}

impl<'a, W, B, C> ReplaceWriter<'a, W, B, C>
where
    W: fmt::Write + ?Sized,
    B: AsRef<str>,
    C: Writeable,
{
    fn new(sink: &'a mut W, needle: &'a B, replacement: &'a C) -> Self {
        Self {
            sink,
            needle,
            replacement,
            matched_chars: 0,
        }
    }

    // Processes a single character from the source stream.
    fn write_char_buffered(&mut self, c: char) -> fmt::Result {
        let needle_str = self.needle.as_ref();
        let needle_len = needle_str.chars().count();

        // If the needle is empty, we just pass through the characters.
        if needle_len == 0 {
            return self.sink.write_char(c);
        }

        let mut j = self.matched_chars;
        // KMP State Transition:
        // While we have a mismatch and we are not at the start of the needle,
        // backtrack using the prefix function.
        while j > 0 && get_char(needle_str, j) != Some(c) {
            let old_j = j;
            j = get_pi_char(needle_str, j - 1);
            // Since we backtracked, the characters in `needle[j..old_j]` are no longer
            // part of the potential match. We must write them to the sink.
            // Note: We only write the prefix of the needle that is no longer matched.
            for ch in needle_str.chars().take(old_j - j) {
                self.sink.write_char(ch)?;
            }
        }

        // If the character matches the next character in the needle, advance the match state.
        if get_char(needle_str, j) == Some(c) {
            j += 1;
            // Check if we found a full match.
            if j == needle_len {
                // Full match found! Write the replacement instead of the needle.
                self.replacement.write_to(self.sink)?;
                // Reset match state to start searching for the next occurrence.
                j = 0;
            }
        } else {
            // Mismatch at the very beginning of the needle. Write the character as is.
            self.sink.write_char(c)?;
        }
        self.matched_chars = j;
        Ok(())
    }

    // Flushes any partially matched prefix at the end of the stream.
    // If the stream ends and we have a partial match, those characters must be written.
    fn flush(&mut self) -> fmt::Result {
        let needle_str = self.needle.as_ref();
        for ch in needle_str.chars().take(self.matched_chars) {
            self.sink.write_char(ch)?;
        }
        self.matched_chars = 0;
        Ok(())
    }
}

impl<'a, W, B, C> fmt::Write for ReplaceWriter<'a, W, B, C>
where
    W: fmt::Write + ?Sized,
    B: AsRef<str>,
    C: Writeable,
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char_buffered(c)?;
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.write_char_buffered(c)
    }
}

impl<A, B, C> Writeable for Replace<A, B, C>
where
    A: Writeable,
    B: AsRef<str>,
    C: Writeable,
{
    fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
        let mut writer = ReplaceWriter::new(sink, &self.needle, &self.replacement);
        self.source.write_to(&mut writer)?;
        writer.flush()
    }

    fn write_to_parts<S: PartsWrite + ?Sized>(&self, sink: &mut S) -> fmt::Result {
        self.write_to(sink)
    }

    fn writeable_length_hint(&self) -> LengthHint {
        let source_hint = self.source.writeable_length_hint();
        let needle_len = self.needle.as_ref().len();
        let replacement_hint = self.replacement.writeable_length_hint();

        if let Some(replacement_len) = replacement_hint.1 {
            if replacement_hint.0 == replacement_len && needle_len == replacement_len {
                return source_hint;
            }
        }

        LengthHint::undefined()
    }
}

impl_display_with_writeable!(Replace<A, B, C>, #[cfg(feature = "alloc")], where A: Writeable, B: AsRef<str>, C: Writeable);

#[test]
fn test_replace() {
    use crate::assert_writeable_eq;
    use crate::concat::Concat;

    // Basic replacement
    let replace1 = Replace {
        source: Concat("Hello", " 10 22 1101 33"),
        needle: "10",
        replacement: Concat("4", "4"),
    };
    assert_writeable_eq!(replace1, "Hello 44 22 1441 33");

    // Empty needle (should just write source)
    let replace2 = Replace {
        source: "Hello World",
        needle: "",
        replacement: "X",
    };
    assert_writeable_eq!(replace2, "Hello World");

    // Empty replacement
    let replace3 = Replace {
        source: "Hello 10 World 10",
        needle: "10",
        replacement: "",
    };
    assert_writeable_eq!(replace3, "Hello  World ");

    // Needle not found
    let replace4 = Replace {
        source: "Hello World",
        needle: "10",
        replacement: "X",
    };
    assert_writeable_eq!(replace4, "Hello World");

    // Needle at the beginning
    let replace5 = Replace {
        source: "10 Hello World",
        needle: "10",
        replacement: "X",
    };
    assert_writeable_eq!(replace5, "X Hello World");

    // Needle at the end
    let replace6 = Replace {
        source: "Hello World 10",
        needle: "10",
        replacement: "X",
    };
    assert_writeable_eq!(replace6, "Hello World X");

    // Overlapping needles (should consume and not match again)
    let replace7 = Replace {
        source: "ababa",
        needle: "aba",
        replacement: "X",
    };
    assert_writeable_eq!(replace7, "Xba");

    // Self-overlap but no match
    let replace8 = Replace {
        source: "aab",
        needle: "aac",
        replacement: "X",
    };
    assert_writeable_eq!(replace8, "aab");

    // Multi-byte UTF-8
    let replace9 = Replace {
        source: "🚀 🛸 🚀🚀 🚁",
        needle: "🚀",
        replacement: "星",
    };
    assert_writeable_eq!(replace9, "星 🛸 星星 🚁");

    // Multi-byte UTF-8 with partial match
    let replace10 = Replace {
        source: "🚀🚁",
        needle: "🚀🛸",
        replacement: "星",
    };
    assert_writeable_eq!(replace10, "🚀🚁");
}
