# Unicode Properties and Code Point Tries in ICU4X

## Background

[Unicode Properties](https://unicode-org.github.io/icu/userguide/strings/properties.html) represent attributes of code points in the Unicode specification. 

Binary properties indicate whether a code point possesses a particular aspect, such as "does this represent whitespace?"

Enumerated properties indicate a two-level identification to a code point, specifically, when a particular aspect is associated with a known finite set of values (as the word "enumerated" implies). For the enumerated property that indicates that a code point is a "line break", one of the values in the associated set of property values might distinguish that a code point is a line-breaking hyphen, while another value might indicate that a code point is a line feed.

Before considering the design of APIs and efficient data structures, we first have to consider the shape of the data. In the binary properties case, there are two dimensions being associated: the binary property and the code point. In enumerated properties, there are three dimensions: the enumerated property, the enumerated property value, and the code point.

The use cases, or manner of data access, inform the design(s) of APIs and data structures. For regular expression parsers (regex), we need to support a text description of a set of code points sharing a property. In this case, returning a [`UnicodeSet`](https://unicode-org.github.io/icu/userguide/strings/unicodeset.html) (a set of Unicode code points) would provide the most efficient usable data. For binary properties, the property name is enough for input. For enumerated properties, the property name and a specific property value are required to uniquely determine a set of code points. In these cases, all dimensions except the code point dimension are fixed (given as inputs).

In other cases, such as the implementation of internationalization algorithms, iteration through code points is a typical implementation strategy. During such iteration, the value of a code point property -- usually, an enumerated property -- can inform the algorithm in question. In such cases, the code point value and enumerated property name dimensions must be fixed (provided as inputs), and the return value is the remaining dimension -- the enumerated property value. To support this use case, the [`CodePointTrie`](https://sites.google.com/site/icusite/design/struct/utrie) data structure is an optimal implementation.

The CodePointTrie data structure also serves Unicode data lookups to serve algorithms for Unicode normalization, collation, etc.

### Notes on Implementation

UnicodeSet represents a set of Unicode code points. The combination of those 2 aspects -- Unicode code point values fill the entire integer range from 0 to 0x10FFFF, and that a set has only 2 values -- together allow for an inversion list implementation that is optimally efficient. An inversion list stores the boundaries of each range (contiguous stretch of code points) that are included in the set. This makes the size of the inversion list range from O(1) to O(n) (and oftentimes O(1)) even when the cardinality of the values logically represented is O(n). Checking for inclusion is just a matter of running binary search on the boundary values and checking if the corresponding inversion list index value is even or odd.

A CodePointTrie is an optimized implementation of what can be represented as an inversion map. An inversion map is similar to an inversion list, except that each range of code points is associated with a value. (By contrast, a range of code points in an inversion list is associated with an implied value of "true" for whether they are included in the set.) 

A CodePointTrie optimizes over a generic inversion map in different ways. One example is that for code point values in the BMP range (16 bits), the 16 bits can be split into the high-order 10 bits and the low-order 6 bits, where the low-order 6 bits can be used as an index into a table/index array. Also, CodePointTries can be created where the binary data values are serialized with 8-bit, 16-bit, and 32-bit encoding, to make lookups more efficient without making encoding conversions.

CodePointTrie code in ICU4C is implemented with a mutable builder, a method to convert the mutable builder to an immutable version, and code to read from the immutable version. The immutable version is stored in memory the same as it is serialized to persistent storage.

## Implementing CodePointTrie in ICU4X 

See previous discussion in [issue 131](https://github.com/unicode-org/icu4x/issues/131) regarding BytesTrie and CodePointTrie, and the [full meeting minutes](https://docs.google.com/document/d/1oaFovJiRbuBG-O9aq0h69gpGxMH0a_pHmMkO5YweS0E/edit#) of the related meeting for further details.

### Options

#### Option 1: Re-implement CodePointTrie in Rust in ICU4X

This option entails writing new Rust code that supports all of the functionality of the CodePointTrie.

The advantages would be to have a Rust-native code that can compile entirely within ICU4X and potentially be used elsewhere.

The disadvantages would be the large amount of time to implement all of the complicated logic (largely located in the builder), and the possibility of losing feature parity 'sync' if/when ICU makes further improvements to the CodePointTrie implementation.

#### Option 2: Implement a reader for the ICU4C CodePointTrie in Rust in ICU4X



### Conclusion

