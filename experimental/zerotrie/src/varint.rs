// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Varint spec for ZeroTrie:
//!
//! - Lead byte: top 2 bits are trie metadata; third is varint extender; rest is value
//! - Trail bytes: top bit is varint extender; add rest to current value * 2^7
//! - Add the "latent value" to the final result: (1<<5) + (1<<7) + (1<<14) + ...

use crate::builder::konst::ConstArrayBuilder;

#[cfg(feature = "alloc")]
use crate::builder::nonconst::TrieBuilderStore;

/// Reads a varint with 2 bits of metadata in the lead byte.
pub const fn read_varint(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    let mut value = (start & 0b00011111) as usize;
    let mut remainder = remainder;
    if (start & 0b00100000) != 0 {
        loop {
            let next;
            (next, remainder) = match remainder.split_first() {
                Some(t) => t,
                None => return None,
            };
            // Note: value << 7 could drop high bits. The first addition can't overflow.
            // The second addition could overflow; in such a case we just inform the
            // developer via the debug assertion.
            value = (value << 7) + ((*next & 0b01111111) as usize) + 32;
            if (*next & 0b10000000) == 0 {
                break;
            }
        }
    }
    Some((value, remainder))
}

/// Reads a varint with 3 bits of metadata in the lead byte.
pub const fn read_extended_varint(start: u8, remainder: &[u8]) -> Option<(usize, &[u8])> {
    let mut value = (start & 0b00001111) as usize;
    let mut remainder = remainder;
    if (start & 0b00010000) != 0 {
        loop {
            let next;
            (next, remainder) = match remainder.split_first() {
                Some(t) => t,
                None => return None,
            };
            // Note: value << 7 could drop high bits. The first addition can't overflow.
            // The second addition could overflow; in such a case we just inform the
            // developer via the debug assertion.
            value = (value << 7) + ((*next & 0b01111111) as usize) + 16;
            if (*next & 0b10000000) == 0 {
                break;
            }
        }
    }
    Some((value, remainder))
}

#[cfg(feature = "alloc")]
pub(crate) fn try_read_extended_varint_from_tstore<S: TrieBuilderStore>(
    start: u8,
    remainder: &mut S,
) -> Option<usize> {
    let mut value = (start & 0b00001111) as usize;
    if (start & 0b00010000) != 0 {
        loop {
            let next = remainder.atbs_split_first()?;
            // Note: value << 7 could drop high bits. The first addition can't overflow.
            // The second addition could overflow; in such a case we just inform the
            // developer via the debug assertion.
            value = (value << 7) + ((next & 0b01111111) as usize) + 16;
            if (next & 0b10000000) == 0 {
                break;
            }
        }
    }
    Some(value)
}

#[cfg(test)]
const MAX_VARINT: usize = usize::MAX;

// *Upper Bound:* Each trail byte stores 7 bits of data, plus the latent value.
// Add an extra 1 since the lead byte holds only 5 bits of data.
const MAX_VARINT_LENGTH: usize = 1 + core::mem::size_of::<usize>() * 8 / 7;

pub(crate) const fn write_varint(value: usize) -> ConstArrayBuilder<MAX_VARINT_LENGTH, u8> {
    let mut result = [0; MAX_VARINT_LENGTH];
    let mut i = MAX_VARINT_LENGTH - 1;
    let mut value = value;
    let mut last = true;
    loop {
        if value < 32 {
            result[i] = value as u8;
            if !last {
                result[i] |= 0b00100000;
            }
            break;
        }
        value -= 32;
        result[i] = (value as u8) & 0b01111111;
        if !last {
            result[i] |= 0b10000000;
        } else {
            last = false;
        }
        value >>= 7;
        i -= 1;
    }
    // The bytes are from i to the end.
    ConstArrayBuilder::from_manual_slice(result, i, MAX_VARINT_LENGTH)
}

pub(crate) const fn write_extended_varint(
    value: usize,
) -> ConstArrayBuilder<MAX_VARINT_LENGTH, u8> {
    let mut result = [0; MAX_VARINT_LENGTH];
    let mut i = MAX_VARINT_LENGTH - 1;
    let mut value = value;
    let mut last = true;
    loop {
        if value < 16 {
            result[i] = value as u8;
            if !last {
                result[i] |= 0b00010000;
            }
            break;
        }
        value -= 16;
        result[i] = (value as u8) & 0b01111111;
        if !last {
            result[i] |= 0b10000000;
        } else {
            last = false;
        }
        value >>= 7;
        i -= 1;
    }
    // The bytes are from i to the end.
    ConstArrayBuilder::from_manual_slice(result, i, MAX_VARINT_LENGTH)
}

/// A secondary implementation that separates the latent value while computing the varint.
#[cfg(test)]
pub(crate) const fn write_varint_reference(
    value: usize,
) -> ConstArrayBuilder<MAX_VARINT_LENGTH, u8> {
    let mut result = [0; MAX_VARINT_LENGTH];
    if value < 32 {
        result[0] = value as u8;
        return ConstArrayBuilder::from_manual_slice(result, 0, 1);
    }
    result[0] = 32;
    let mut latent = 32;
    let mut steps = 2;
    loop {
        let next_latent = (latent << 7) + 32;
        if value < next_latent || next_latent == latent {
            break;
        }
        latent = next_latent;
        steps += 1;
    }
    let mut value = value - latent;
    let mut i = steps;
    while i > 0 {
        i -= 1;
        result[i] |= (value as u8) & 0b01111111;
        value >>= 7;
        if i > 0 && i < steps - 1 {
            result[i] |= 0b10000000;
        }
    }
    // The bytes are from 0 to `steps`.
    ConstArrayBuilder::from_manual_slice(result, 0, steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestCase<'a> {
        bytes: &'a [u8],
        remainder: &'a [u8],
        value: usize,
    }
    static CASES: &[TestCase] = &[
        TestCase {
            bytes: &[0b00000000],
            remainder: &[],
            value: 0,
        },
        TestCase {
            bytes: &[0b00001010],
            remainder: &[],
            value: 10,
        },
        TestCase {
            bytes: &[0b00011111],
            remainder: &[],
            value: 31,
        },
        TestCase {
            bytes: &[0b00011111, 0b10101010],
            remainder: &[0b10101010],
            value: 31,
        },
        TestCase {
            bytes: &[0b00100000, 0b00000000],
            remainder: &[],
            value: 32,
        },
        TestCase {
            bytes: &[0b00100000, 0b00000001],
            remainder: &[],
            value: 33,
        },
        TestCase {
            bytes: &[0b00100000, 0b00100000],
            remainder: &[],
            value: 64,
        },
        TestCase {
            bytes: &[0x20, 0x44],
            remainder: &[],
            value: 100,
        },
        TestCase {
            bytes: &[0b00100000, 0b01111111],
            remainder: &[],
            value: 159,
        },
        TestCase {
            bytes: &[0b00100001, 0b00000000],
            remainder: &[],
            value: 160,
        },
        TestCase {
            bytes: &[0b00100001, 0b00000001],
            remainder: &[],
            value: 161,
        },
        TestCase {
            bytes: &[0x23, 0x54],
            remainder: &[],
            value: 500,
        },
        TestCase {
            bytes: &[0b00111111, 0b01111111],
            remainder: &[],
            value: 4127, // 32 + (1 << 12) - 1
        },
        TestCase {
            bytes: &[0b00100000, 0b10000000, 0b00000000],
            remainder: &[],
            value: 4128, // 32 + (1 << 12)
        },
        TestCase {
            bytes: &[0b00100000, 0b10000000, 0b00000001],
            remainder: &[],
            value: 4129, // 32 + (1 << 12) + 1
        },
        TestCase {
            bytes: &[0b00100000, 0b10000000, 0b01111111],
            remainder: &[],
            value: 4255, // 32 + (1 << 12) + 127
        },
        TestCase {
            bytes: &[0b00100000, 0b10000001, 0b00000000],
            remainder: &[],
            value: 4256, // 32 + (1 << 12) + 128
        },
        TestCase {
            bytes: &[0b00100000, 0b10000001, 0b00000001],
            remainder: &[],
            value: 4257, // 32 + (1 << 12) + 129
        },
        TestCase {
            bytes: &[0x20, 0x86, 0x68],
            remainder: &[],
            value: 5000,
        },
        TestCase {
            bytes: &[0b00100000, 0b11111111, 0b01111111],
            remainder: &[],
            value: 20511, // 32 + (1 << 12) + (1 << 14) - 1
        },
        TestCase {
            bytes: &[0b00100001, 0b10000000, 0b00000000],
            remainder: &[],
            value: 20512, // 32 + (1 << 12) + (1 << 14)
        },
        TestCase {
            bytes: &[0b00111111, 0b11111111, 0b01111111],
            remainder: &[],
            value: 528415, // 32 + (1 << 12) + (1 << 19) - 1
        },
        TestCase {
            bytes: &[0b00100000, 0b10000000, 0b10000000, 0b00000000],
            remainder: &[],
            value: 528416, // 32 + (1 << 12) + (1 << 19)
        },
        TestCase {
            bytes: &[0b00100000, 0b10000000, 0b10000000, 0b00000001],
            remainder: &[],
            value: 528417, // 32 + (1 << 12) + (1 << 19) + 1
        },
        TestCase {
            bytes: &[0b00111111, 0b11111111, 0b11111111, 0b01111111],
            remainder: &[],
            value: 67637279, // 32 + (1 << 12) + (1 << 19) + (1 << 26) - 1
        },
        TestCase {
            bytes: &[0b00100000, 0b10000000, 0b10000000, 0b10000000, 0b00000000],
            remainder: &[],
            value: 67637280, // 32 + (1 << 12) + (1 << 19) + (1 << 26)
        },
    ];

    #[test]
    fn test_read() {
        for cas in CASES {
            let recovered = read_varint(cas.bytes[0], &cas.bytes[1..]).unwrap();
            assert_eq!(recovered, (cas.value, cas.remainder), "{:?}", cas);
        }
    }

    #[test]
    fn test_read_write() {
        for cas in CASES {
            let reference_bytes = write_varint_reference(cas.value);
            assert_eq!(
                reference_bytes.len(),
                cas.bytes.len() - cas.remainder.len(),
                "{:?}",
                cas
            );
            assert_eq!(
                reference_bytes.as_slice(),
                &cas.bytes[0..reference_bytes.len()],
                "{:?}",
                cas
            );
            let recovered = read_varint(cas.bytes[0], &cas.bytes[1..]).unwrap();
            assert_eq!(recovered, (cas.value, cas.remainder), "{:?}", cas);
            let write_bytes = write_varint(cas.value);
            assert_eq!(
                reference_bytes.as_slice(),
                write_bytes.as_slice(),
                "{:?}",
                cas
            );
        }
    }

    #[test]
    fn test_roundtrip() {
        let mut i = 0usize;
        while i < MAX_VARINT {
            let bytes = write_varint(i);
            let recovered = read_varint(bytes.as_slice()[0], &bytes.as_slice()[1..]);
            assert!(recovered.is_some(), "{:?}", i);
            assert_eq!(i, recovered.unwrap().0, "{:?}", bytes.as_slice());
            i <<= 1;
            i += 1;
        }
    }

    #[test]
    fn test_max() {
        let reference_bytes = write_varint_reference(MAX_VARINT);
        let write_bytes = write_varint(MAX_VARINT);
        assert_eq!(reference_bytes.len(), MAX_VARINT_LENGTH);
        assert_eq!(reference_bytes.as_slice(), write_bytes.as_slice());
        let subarray = write_bytes
            .as_const_slice()
            .get_subslice_or_panic(1, write_bytes.len());
        let (recovered_value, remainder) = read_varint(
            *write_bytes.as_const_slice().first().unwrap(),
            subarray.as_slice(),
        )
        .unwrap();
        assert!(remainder.is_empty());
        assert_eq!(recovered_value, MAX_VARINT);
        assert_eq!(
            write_bytes.as_slice(),
            &[
                0b00100001, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b11011111, //
                0b01011111, //
            ]
        );
    }
}
