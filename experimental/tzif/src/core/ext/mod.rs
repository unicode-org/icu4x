// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A trait extension for when the source input item is a byte.
pub mod byte;

/// A trait extension for chaining parse functions to collect multiple values.
pub mod chain;

/// A trait extension for defaulting to values after failure.
pub mod default;

/// A trait extension to expect values but not return them.
pub mod expect;

/// A trait extension to inspect values without advancing the input.
pub mod inspect;

/// A trait extension to parse many values at a time.
pub mod many;

/// A trait extension to map a parsed value of the source parse input.
pub mod map;

/// A trait extension to handle expected parse failure.
pub mod maybe;

/// A trait extension to require parsed items to match predicates.
pub mod require;

/// A trait extension to invoke behavior after parsing a value.
pub mod then;

#[cfg(test)]
mod test {
    use crate::{
        core::{
            ext::{chain::Chain, many::ParseMany, maybe::MaybeParse, require::Require, then::Then},
            ParseInput,
        },
        reader::slice::SliceByteReader,
    };

    #[test]
    fn maybe_parse() {
        let mut source = SliceByteReader::from_str("abcdefg");

        let parsed = source
            .maybe_parse(|source| source.require_next(|&byte| byte == b'z'))
            .unwrap();
        assert_eq!(parsed.value(), &None);

        let mut parsed = source
            .maybe_parse(|source| source.require_next(|&byte| byte == b'a'))
            .unwrap();
        assert_eq!(parsed.value(), &Some(b'a'));

        let parsed = parsed
            .begin_chain()
            .chain(|source| source.maybe_parse(|source| source.require_next(|&byte| byte == b'b')))
            .chain(|source| source.maybe_parse(|source| source.require_next(|&byte| byte == b'c')))
            .chain(|source| source.maybe_parse(|source| source.require_next(|&byte| byte == b'z')))
            .chain(|source| source.maybe_parse(|source| source.require_next(|&byte| byte == b'y')))
            .chain(|source| source.maybe_parse(|source| source.require_next(|&byte| byte == b'd')))
            .unwrap();

        assert_eq!(
            parsed.value(),
            &(Some(b'b'), Some(b'c'), None, None, Some(b'd'),),
        );
    }

    #[test]
    fn require_next() {
        let mut source = SliceByteReader::from_str("abcdefg");

        assert!(source.require_next(|&byte| byte == b'0').is_err());

        let mut parsed = source
            .require_next(|&byte| byte == b'a')
            .require_next(|&byte| byte == b'b')
            .unwrap();

        assert_eq!(*parsed.value(), b'b');
        assert!(!parsed.end_of_input());
        assert!(!source.end_of_input());

        let mut rest = parsed
            .parse_zero_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"cdefg"));

        assert!(rest.is_ok());
        assert!(rest.end_of_input());
        assert!(!parsed.end_of_input());
        assert!(!source.end_of_input());
    }

    #[test]
    fn require_take() {
        let mut source = SliceByteReader::from_str("abcdefg");

        assert!(source.require_take(4, |bytes| bytes == b"ab").is_err());
        assert!(!source.end_of_input());

        let mut parsed = source.require_take(4, |bytes| bytes == b"abcd").unwrap();
        assert!(!parsed.end_of_input());
        assert!(!source.end_of_input());

        let mut rest = parsed
            .parse_zero_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"efg"));

        assert!(rest.is_ok());
        assert!(rest.end_of_input());
        assert!(!parsed.end_of_input());
        assert!(!source.end_of_input());
    }

    #[test]
    fn test_zero_or_more() {
        let mut source = SliceByteReader::from_str("abcdefg");
        let mut parsed = source
            .parse_zero_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"abcdefg"));

        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());

        let mut parsed2 = parsed
            .parse_zero_or_more(|source| source.next())
            .then(|bytes| assert!(bytes.is_empty()));

        assert!(parsed2.end_of_input());
        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());
    }

    #[test]
    fn test_one_or_more() {
        let mut source = SliceByteReader::from_str("abcdefg");
        let mut parsed = source
            .parse_one_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"abcdefg"));

        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());

        let parsed2 = parsed
            .parse_one_or_more(|source| source.next())
            .then(|bytes| assert!(bytes.is_empty()));

        assert!(parsed2.is_err());
        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());
    }

    #[test]
    fn skip() {
        let mut source = SliceByteReader::from_str("abcdefg");
        let mut parsed = source
            .skip(0)
            .parse_one_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"abcdefg"));

        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());

        let mut parsed2 = source
            .skip(2)
            .parse_one_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"cdefg"));

        assert!(parsed2.end_of_input());
        assert!(!source.end_of_input());
    }

    #[test]
    fn skip_while() {
        let mut source = SliceByteReader::from_str("abcdefg");
        let mut parsed = source
            .skip_while(|&byte| byte < b'd')
            .then(|&skipped| assert_eq!(skipped, 3))
            .parse_one_or_more(|source| source.next())
            .then(|bytes| assert_eq!(bytes, b"defg"));

        assert!(parsed.end_of_input());
        assert!(!source.end_of_input());

        let mut parsed2 = source
            .skip_while(|&byte| byte < b'i')
            .then(|&skipped| assert_eq!(skipped, 7))
            .parse_zero_or_more(|s| s.next())
            .then(|bytes| assert_eq!(bytes, &[0u8; 0]));

        assert!(parsed2.end_of_input());
        assert!(!source.end_of_input());
    }
}
