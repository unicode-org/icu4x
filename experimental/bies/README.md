# bies [![crates.io](https://img.shields.io/crates/v/bies)](https://crates.io/crates/bies)

<!-- cargo-rdme start -->

The algorithms in this project convert from a BIES matrix (the output of the LSTM segmentation neural network) to concrete segment boundaries.  In BIES, B = beginning of segment; I = inside segment; E = end of segment; and S = single segment (both beginning and end).

These algorithms always produce valid breakpoint positions (at grapheme cluster boundaries); they don't assume that the neural network always predicts valid positions.

## Example

For example, suppose you had the following BIES matrix:

<pre>
|   B   |   I   |   E   |   S   |
|-------|-------|-------|-------|
| 0.01  | 0.01  | 0.01  | 0.97  |
| 0.97  | 0.01  | 0.01  | 0.01  |
| 0.01  | 0.97  | 0.01  | 0.01  |
| 0.01  | 0.97  | 0.01  | 0.01  |
| 0.01  | 0.01  | 0.97  | 0.01  |
| 0.01  | 0.01  | 0.01  | 0.97  |
| 0.97  | 0.01  | 0.01  | 0.01  |
| 0.01  | 0.01  | 0.97  | 0.01  |
</pre>

This matrix resolves to:

<pre>
01234567
SBIIESBE
</pre>

The breakpoints are then: 0, 1, 5, and 8 (four segments).

However, it could be the case that the algorithm's BIES are invalid.  For example, "BEE" is invalid, because the second "E" does not terminate any word.  The purpose of the algorithms in this project is to guarantee that valid breakpoints and BIES are always outputted.

## Algorithms

The following algorithms are implemented:

**1a:** Step through each grapheme cluster boundary in the string. Look at the BIES vectors for the code points surrounding the boundary. The only valid results at that boundary are {EB, ES, SB, SS} (breakpoint) or {II, BI, IE, BE} (no breakpoint). Take the sum of the valid breakpoint and no-breakpoint probabilities, and decide whether to insert a breakpoint based on which sum is higher. Repeat for all grapheme cluster boundaries in the string. The output is a list of word boundaries, which can be converted back into BIES if desired.

**1b:** Same as 1a, but instead of taking the sum, take the individual maximum.

**2a:** Step through each element in the BIES sequence. For each element, look at the triplet containing the element and both of its neighbors. By induction, assume the first element in the triplet is correct. Now, depending on whether there is a code point boundary following the element, calculate the probabilities of all valid BIES for the triplet, and based on those results, pick the most likely value for the current element.

**3a:** Exhaustively check the probabilities of all possible BIES for the string. This algorithm has exponential runtime.

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
