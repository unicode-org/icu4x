# The `icu_normalizer` Trie Value Format

## General

The trie values are 32-bit values.

0xFFFFFFFF (all bits set to 1) marks UTS 46 ignorables and is not supported in NFD or NFKD data.

## Common Flags

Two flags common to all trie value types other than the above ignorable marker:

Bit 31 (the most significant bit): 1 iff the first character of the decomposition can combine backwards.

Bit 30: 1 if applying NFC to the decomposition does not result in the character being decomposed or (in the jamo case) is otherwise worth keeping off the passthrough track. (Currently, this bit isn't actually useful for non-starters, and a future change might involve setting this flag on non-starters that decompose to themselves if that turns out to be useful for some optimization.)

## Types of Trie Values

The character is a starter (CCC == 0) that decomposes to itself: The 30 lower bits set to zero.

REPLACEMENT CHARACTER: Bit 31 set to 1 and all others set to zero. This in an exception to the above item in order to allow catching UTF-8 errors as a side effect of a passthrough check.

The character is a non-starter (CCC != 0) that decomposes to itself: The highest bit is set to 1, the rest of the high half is set to zeros, the second-least-significant byte is 0xD8, and the least-significant byte is the CCC value.

The character is a non-starter that decomposes to multiple characters such that the first character is a non-starter: The two highest bits are set to 1, the rest of the high half is set to zeros, the second-least-significant byte is 0xD9, and the least-significant byte is the CCC value of the _undecomposed_ character. (The actual decomposition is hard-coded.)

The decomposition is the NFKD decomposition of U+FDFA: The highest bit is 0, the second-highest is 1, the lowest bit is 1, and the rest are zeros. (The actual decomposition is hard-coded.) The lowest bit is deliberately unified with Hangul syllables below to maximize the options for reordering the Hangul check relative to other checks.

Hangul syllable: The trie value is 1. (I.e. only the lowest bit is set to 1.)

The decomposition is a singleton decomposition to a single different BMP starter > 0x1F: The highest bit is 1 iff the decomposition can combine backwards (does not occur as of Unicode 16.0), the second-highest bit is 1, the low half is the decomposition.

The character is not the ANGSTROM SIGN, and the decomposition is to a starter <= U+7FFF and > 0x1F that cannot combine backwards followed by a non-starter <= U+7FFF and > 0x1F: The highest bit is 0. The second-highest bit is set according to its general semantics. The lowest 15 bits are the leading starter. The next 15 bits are the trailing non-starter.

Otherwise: This is a complex decomposition. It must start with a starter, which is theoretically not future-proof but is likely practically going to be OK. The two highest bits are set according to their general semantics. The lower 12 bits of the higher half are the _offset_ to the logical sequence of scalars16, scalars32, supplementary_scalars16, supplementary_scalars32. (The 14 lowest bits are interpreted as the length for forward compatibility, but technically the two highest of these are reserved and could be assigned to flags instead in the future.) The lowest 3 bits are _length_ minus 2 if all characters of the decomposition are within the BMP and _length_ minus 1 otherwise. The fourth-lowest bit is reserved and set to 0. (The four lowest bits are interpreted as length for forward-compatibility, but technically the fourth bit could be allocated to a flag instead.) The fifth bit is set to 1 if all the trailing characters are non-starters. If it is 0, there are no guarantees on the non-starterness of the trailing characters. The 11 highest bits of the lower half are set to zero.

## Additional Tweaks

In NFD and NFKD data, each surrogate has a singleton decomposition to U+FFFD. (This is not included in UTS 46 data, because this is used for UTF-16 slice mode optimization and UTS 46 is only supported in the iterator mode, because the UTS 46 ignorable marker is not supported in the slice mode.)

In UTS 46 data, each disallowed character has a singleton decomposition to U+FFFD.

