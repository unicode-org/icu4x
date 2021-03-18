# Unicode Properties and Code Point Tries in ICU4X

## Background

[Unicode Properties](https://unicode-org.github.io/icu/userguide/strings/properties.html) represent attributes of code points in the Unicode specification. 

Binary properties indicate whether a code point possesses a particular aspect, such as "does this represent whitespace?"

Enumerated properties indicate a two-level identification to a code point, specifically, when a particular aspect is associated with a known finite set of values (as the word "enumerated" implies). For the enumerated property that indicates that a code point is a "line break", one of the values in the associated set of property values might distinguish that a code point is a line-breaking hyphen, while another value might indicate that a code point is a line feed.

Before considering the design of APIs and efficient data structures, we first have to consider the shape of the data. In the binary properties case, there are two dimensions being associated: the binary property and the code point. In enumerated properties, there are three dimensions: the enumerated property, the enumerated property value, and the code point.

The use cases, or manner of data access, inform the design(s) of APIs and data structures. For regular expression parsers (regex), we need to support a text description of a set of code points sharing a property. In this case, returning a [`UnicodeSet`](https://unicode-org.github.io/icu/userguide/strings/unicodeset.html) (a set of Unicode code points) would provide the most efficient usable data. For binary properties, the property name is enough for input. For enumerated properties, the property name and a specific property value are required to uniquely determine a set of code points. In these cases, all dimensions except the code point dimension are fixed by the input value.

In other cases, such as the implementation of internationalization algorithms, iteration through code points is a typical implementation strategy. During such iteration, the value of a code point property -- usually, an enumerated property -- can inform the algorithm in question. In such cases, the code point value and enumerated property name dimensions must be fixed, allowing the enumerated property value to be a return value that can vary.