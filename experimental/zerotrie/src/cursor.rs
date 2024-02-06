// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::reader;
use crate::ZeroAsciiIgnoreCaseTrie;
use crate::ZeroTrieSimpleAscii;

use core::fmt;

impl<Store> ZeroTrieSimpleAscii<Store>
where
    Store: AsRef<[u8]> + ?Sized,
{
    /// Gets a cursor into the current trie.
    ///
    /// Useful to query a trie with data that is not a slice.
    ///
    /// This is currently supported only on [`ZeroTrieSimpleAscii`]
    /// and [`ZeroAsciiIgnoreCaseTrie`].
    ///
    /// # Examples
    ///
    /// Get a value out of a trie by [writing](fmt::Write) it to the cursor:
    ///
    /// ```
    /// use core::fmt::Write;
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "abc" and "abcdef"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"abc\x80def\x81");
    ///
    /// // Get out the value for "abc"
    /// let mut cursor = trie.cursor();
    /// write!(&mut cursor, "abc");
    /// assert_eq!(cursor.take_value(), Some(0));
    /// ```
    ///
    /// Find the longest prefix match:
    ///
    /// ```
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "abc" and "abcdef"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"abc\x80def\x81");
    ///
    /// // Find the longest prefix of the string "abcdxy":
    /// let query = b"abcdxy";
    /// let mut longest_prefix = 0;
    /// let mut cursor = trie.cursor();
    /// for (i, b) in query.iter().enumerate() {
    ///     // Checking is_empty() is not required, but it is
    ///     // good for efficiency
    ///     if cursor.is_empty() {
    ///         break;
    ///     }
    ///     if cursor.take_value().is_some() {
    ///         longest_prefix = i;
    ///     }
    ///     cursor.step(*b);
    /// }
    ///
    /// // The longest prefix is "abc" which is length 3:
    /// assert_eq!(longest_prefix, 3);
    /// ```
    #[inline]
    pub fn cursor(&self) -> ZeroTrieSimpleAsciiCursor {
        ZeroTrieSimpleAsciiCursor {
            trie: self.as_borrowed_slice(),
        }
    }
}

impl<Store> ZeroAsciiIgnoreCaseTrie<Store>
where
    Store: AsRef<[u8]> + ?Sized,
{
    /// Gets a cursor into the current trie.
    ///
    /// Useful to query a trie with data that is not a slice.
    ///
    /// This is currently supported only on [`ZeroTrieSimpleAscii`]
    /// and [`ZeroAsciiIgnoreCaseTrie`].
    ///
    /// # Examples
    ///
    /// Get a value out of a trie by [writing](fmt::Write) it to the cursor:
    ///
    /// ```
    /// use core::fmt::Write;
    /// use zerotrie::ZeroAsciiIgnoreCaseTrie;
    ///
    /// // A trie with two values: "aBc" and "aBcdEf"
    /// let trie = ZeroAsciiIgnoreCaseTrie::from_bytes(b"aBc\x80dEf\x81");
    ///
    /// // Get out the value for "abc" (case-insensitive!)
    /// let mut cursor = trie.cursor();
    /// write!(&mut cursor, "abc");
    /// assert_eq!(cursor.take_value(), Some(0));
    /// ```
    ///
    /// For more examples, see [`ZeroTrieSimpleAscii::cursor`].
    #[inline]
    pub fn cursor(&self) -> ZeroAsciiIgnoreCaseTrieCursor {
        ZeroAsciiIgnoreCaseTrieCursor {
            trie: self.as_borrowed_slice(),
        }
    }
}

impl<'a> ZeroTrieSimpleAscii<&'a [u8]> {
    /// Same as [`ZeroTrieSimpleAscii::cursor()`] but moves self to avoid
    /// having to doubly anchor the trie to the stack.
    #[inline]
    pub fn into_cursor(self) -> ZeroTrieSimpleAsciiCursor<'a> {
        ZeroTrieSimpleAsciiCursor { trie: self }
    }
}

impl<'a> ZeroAsciiIgnoreCaseTrie<&'a [u8]> {
    /// Same as [`ZeroAsciiIgnoreCaseTrie::cursor()`] but moves self to avoid
    /// having to doubly anchor the trie to the stack.
    #[inline]
    pub fn into_cursor(self) -> ZeroAsciiIgnoreCaseTrieCursor<'a> {
        ZeroAsciiIgnoreCaseTrieCursor { trie: self }
    }
}

/// A cursor into a [`ZeroTrieSimpleAscii`], useful for stepwise lookup.
///
/// For examples, see [`ZeroTrieSimpleAscii::cursor()`].
// Clone but not Copy: <https://stackoverflow.com/q/32324251/1407170>
#[derive(Debug, Clone)]
pub struct ZeroTrieSimpleAsciiCursor<'a> {
    trie: ZeroTrieSimpleAscii<&'a [u8]>,
}

/// A cursor into a [`ZeroAsciiIgnoreCaseTrie`], useful for stepwise lookup.
///
/// For examples, see [`ZeroAsciiIgnoreCaseTrie::cursor()`].
// Clone but not Copy: <https://stackoverflow.com/q/32324251/1407170>
#[derive(Debug, Clone)]
pub struct ZeroAsciiIgnoreCaseTrieCursor<'a> {
    trie: ZeroAsciiIgnoreCaseTrie<&'a [u8]>,
}

impl<'a> ZeroTrieSimpleAsciiCursor<'a> {
    /// Steps the cursor one byte into the trie.
    ///
    /// # Examples
    ///
    /// Unrolled loop checking for string presence at every step:
    ///
    /// ```
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "abc" and "abcdef"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"abc\x80def\x81");
    ///
    /// // Search the trie for the string "abcdxy"
    /// let mut cursor = trie.cursor();
    /// assert_eq!(cursor.take_value(), None); // ""
    /// cursor.step(b'a');
    /// assert_eq!(cursor.take_value(), None); // "a"
    /// cursor.step(b'b');
    /// assert_eq!(cursor.take_value(), None); // "ab"
    /// cursor.step(b'c');
    /// assert_eq!(cursor.take_value(), Some(0)); // "abc"
    /// cursor.step(b'd');
    /// assert_eq!(cursor.take_value(), None); // "abcd"
    /// assert!(!cursor.is_empty());
    /// cursor.step(b'x'); // no strings have the prefix "abcdx"
    /// assert!(cursor.is_empty());
    /// assert_eq!(cursor.take_value(), None); // "abcdx"
    /// cursor.step(b'y');
    /// assert_eq!(cursor.take_value(), None); // "abcdxy"
    /// ```
    ///
    /// If the byte is not ASCII, the cursor will become empty:
    ///
    /// ```
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "abc" and "abcdef"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"abc\x80def\x81");
    ///
    /// let mut cursor = trie.cursor();
    /// assert_eq!(cursor.take_value(), None); // ""
    /// cursor.step(b'a');
    /// assert_eq!(cursor.take_value(), None); // "a"
    /// cursor.step(b'b');
    /// assert_eq!(cursor.take_value(), None); // "ab"
    /// cursor.step(b'\xFF');
    /// assert!(cursor.is_empty());
    /// assert_eq!(cursor.take_value(), None);
    /// ```
    #[inline]
    pub fn step(&mut self, byte: u8) {
        reader::step_parameterized::<ZeroTrieSimpleAscii<[u8]>>(&mut self.trie.store, byte);
    }

    /// Takes the value at the current position.
    ///
    /// Calling this function on a new cursor is equivalent to calling `.get()`
    /// with the empty string (except that it can only be called once).
    ///
    /// # Examples
    ///
    /// ```
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "" and "abc"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"\x80abc\x81");
    ///
    /// assert_eq!(Some(0), trie.get(""));
    /// let mut cursor = trie.cursor();
    /// assert_eq!(Some(0), cursor.take_value());
    /// assert_eq!(None, cursor.take_value());
    /// ```
    #[inline]
    pub fn take_value(&mut self) -> Option<usize> {
        reader::take_value(&mut self.trie.store)
    }

    /// Checks whether the cursor points to an empty trie.
    ///
    /// Use this to determine when to stop iterating.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.trie.is_empty()
    }
}

impl<'a> ZeroAsciiIgnoreCaseTrieCursor<'a> {
    /// Steps the cursor one byte into the trie.
    ///
    /// Returns the byte if matched, which may be a different case than the input byte.
    /// If this function returns `None`, any lookup loops can be terminated.
    ///
    /// # Examples
    ///
    /// Normalize the case of a value by stepping through an ignore-case trie:
    ///
    /// ```
    /// use core::fmt::Write;
    /// use std::borrow::Cow;
    /// use zerotrie::ZeroAsciiIgnoreCaseTrie;
    ///
    /// // A trie with two values: "aBc" and "aBcdEf"
    /// let trie = ZeroAsciiIgnoreCaseTrie::from_bytes(b"aBc\x80dEf\x81");
    ///
    /// // Get out the value for "abc" and normalize the key string
    /// let mut cursor = trie.cursor();
    /// let mut key_str = Cow::Borrowed("abc".as_bytes());
    /// let mut i = 0;
    /// let value = loop {
    ///     if i == key_str.len() {
    ///         break cursor.take_value();
    ///     }
    ///     let input_byte = key_str[i];
    ///     let Some(matched_byte) = cursor.step(input_byte) else {
    ///         break None;
    ///     };
    ///     if matched_byte != input_byte {
    ///         key_str.to_mut()[i] = matched_byte;
    ///     }
    ///     i += 1;
    /// };
    ///
    /// assert_eq!(value, Some(0));
    /// assert_eq!(&*key_str, "aBc".as_bytes());
    /// ```
    ///
    /// For more examples, see [`ZeroAsciiIgnoreCaseTrieCursor::step`].
    #[inline]
    pub fn step(&mut self, byte: u8) -> Option<u8> {
        reader::step_parameterized::<ZeroAsciiIgnoreCaseTrie<[u8]>>(&mut self.trie.store, byte)
    }

    /// Takes the value at the current position.
    ///
    /// For more details, see [`ZeroAsciiIgnoreCaseTrieCursor::take_value`].
    #[inline]
    pub fn take_value(&mut self) -> Option<usize> {
        reader::take_value(&mut self.trie.store)
    }

    /// Checks whether the cursor points to an empty trie.
    ///
    /// For more details, see [`ZeroAsciiIgnoreCaseTrieCursor::is_empty`].
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.trie.is_empty()
    }
}

impl<'a> fmt::Write for ZeroTrieSimpleAsciiCursor<'a> {
    /// Steps the cursor through each ASCII byte of the string.
    ///
    /// If the string contains non-ASCII chars, an error is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::fmt::Write;
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "abc" and "abcdef"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"abc\x80def\x81");
    ///
    /// let mut cursor = trie.cursor();
    /// cursor.write_str("abcdxy").expect("all ASCII");
    /// cursor.write_str("🚂").expect_err("non-ASCII");
    /// ```
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if !b.is_ascii() {
                return Err(fmt::Error);
            }
            self.step(b);
        }
        Ok(())
    }

    /// Equivalent to [`ZeroTrieSimpleAsciiCursor::step()`], except returns
    /// an error if the char is non-ASCII.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::fmt::Write;
    /// use zerotrie::ZeroTrieSimpleAscii;
    ///
    /// // A trie with two values: "abc" and "abcdef"
    /// let trie = ZeroTrieSimpleAscii::from_bytes(b"abc\x80def\x81");
    ///
    /// let mut cursor = trie.cursor();
    /// cursor.write_char('a').expect("ASCII");
    /// cursor.write_char('x').expect("ASCII");
    /// cursor.write_char('🚂').expect_err("non-ASCII");
    /// ```
    fn write_char(&mut self, c: char) -> fmt::Result {
        if !c.is_ascii() {
            return Err(fmt::Error);
        }
        self.step(c as u8);
        Ok(())
    }
}

impl<'a> fmt::Write for ZeroAsciiIgnoreCaseTrieCursor<'a> {
    /// Steps the cursor through each ASCII byte of the string.
    ///
    /// If the string contains non-ASCII chars, an error is returned.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.bytes() {
            if !b.is_ascii() {
                return Err(fmt::Error);
            }
            self.step(b);
        }
        Ok(())
    }

    /// Equivalent to [`ZeroAsciiIgnoreCaseTrieCursor::step()`], except returns
    /// an error if the char is non-ASCII.
    fn write_char(&mut self, c: char) -> fmt::Result {
        if !c.is_ascii() {
            return Err(fmt::Error);
        }
        self.step(c as u8);
        Ok(())
    }
}
