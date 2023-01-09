Format to Parts in ICU4X
========================

ECMA-402 specifies *[formatToParts](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/formatToParts#Description)*, a variant of the *format* function that returns the string split into segments according to what the segment represents.  For example:

```javascript
new Intl.DateTimeFormat("fr").format(new Date())
// "19/11/2020"

new Intl.DateTimeFormat("fr").formatToParts(new Date())
/* [
  {type: "day", value: "19"},
  {type: "literal", value: "/"},
  {type: "month", value: "11"},
  {type: "literal", value: "/"},
  {type: "year", value: "2020"},
] */

new Intl.DateTimeFormat("fr", { dateStyle: "medium" }).formatRange(new Date(2020, 0, 1), new Date(2020, 2, 3))
// "1 janv. – 3 mars 2020"

new Intl.DateTimeFormat("fr", { dateStyle: "medium" }).formatRangeToParts(new Date(2020, 0, 1), new Date(2020, 2, 3))
/* [
  {type: "day", value: "1", source: "startRange"},
  {type: "literal", value: " ", source: "startRange"},
  {type: "month", value: "janv.", source: "startRange"},
  {type: "literal", value: " – ", source: "shared"},
  {type: "day", value: "3", source: "endRange"},
  {type: "literal", value: " ", source: "endRange"},
  {type: "month", value: "mars", source: "endRange"},
  {type: "literal", value: " ", source: "shared"},
  {type: "year", value: "2020", source: "shared"},
] */
```

The goal of this document is to discuss how we will deal with `formatToParts` in ICU4X, both with regard to the internal data model (capabilities) and the internal data model (implementation).

## External Data Model

There are two prevailing models in formatting to parts: linear attributes and nested fields.

### Linear Attributes

ECMA-402 uses the linear attributes model: each substring is mapped to exactly one bag of attributes.  Expressed in terms of starts and ends of ranges:

```
1 janv. – 3 mars 2020

[) day, startRange
 [) literal, startRange
  [    ) month, startRange
       [  ) literal, shared
          [) day, endRange
           [) literal, endRange
            [   ) month, endRange
                [) literal, shared
                 [   ) year, Shared
```

### Nested Fields

Parts of ICU use the nested fields model: there is a set of ranges, and a particular character could live in more than one range.  Here is how the above date interval format would look with nested fields:

```
1 janv. – 3 mars 2020

[) day
 [) literal
  [    ) month
[      ) startRange
       [  ) literal
       [  ) shared
          [) day
           [) literal
            [   ) month
          [     ) endRange
                [) literal
                 [   ) year
                [    ) shared
```

The linear attributes model is simpler, but the nested fields model is more flexible.

## Internal Data Model

This section discusses alternate representations of `formatToParts` data with respect to the internal representation in types such as `FormattedDateTime`.

### Model A

Keep two parallel arrays, one with code points and the other with a 1-char field identifier (which could be an integer instead of a char).

```
chars  = "19/11/2020"
fields = [dd0mm0yyyy]
```

To convert from this data model to the `formatToParts` return value, coalesce adjacent field identifiers into ranges.  In the above example, there are 5 ranges:

1. d \[0,2)
2. 0 \[2,3)
3. m \[3,5)
4. 0 \[5,6)
5. y \[6,10)

Pros:

- **Efficient:** Code size and performance are not substantially affected by the addition of the parallel fields array to FormattedDateTime. For example, substrings can be inserted into the FormattedDateTime without having to update a separate index-based structure.
- **Linear:** The data model directly corresponds to a 1-to-1 linear mapping between code points and fields.

Cons:

- **Doesn't support multiple attributes or nested fields:** Additional business logic is required to support all but the most basic field logic.  For example, "2020" is a year, a number, and an integer, but this data model is only able to represent one type.  This problem comes up in situations like DateIntervalFormat, where ECMA-402 requires the implementation to return multiple attributes on a particular `formatToParts` object.
- **Doesn't distinguish adjacent fields:** Since adjacent field identifiers are coalesced, this data model doesn't support situations where two fields of the same type are adjacent to each other in the string.  For example, in Chinese, no separators are used in certain types of lists, so different list items cannot be distinguished without an extra data structure.

### Model B

Keep a chars array plus a second array that contains field startpoints and endpoints.

```
chars = "19/11/2020"
fields = [
  { field: 'd', start: 0, end: 2 },
  { field: '0', start: 2, end: 3 },
  { field: 'm', start: 3, end: 5 },
  { field: '0', start: 5, end: 6 },
  { field: 'y', start: 6, end: 10 },
]
```

Pros:

- **Generalizable:** Supports nested fields, ability to add metadata to fields, and ability to have adjacent fields.

Cons:

- **Slow on insert/prepend:** This model works when the only operation is to append to the end of a string, but in i18n formatting, prepend and insert are common operations that require extra care with this data model; see Appendix A.  Imposing this restriction on ICU4X would mean a choice between a quadratic field updating algorithm on all formatting operations, or complicated feature-flagging to disable the field structure when it is not needed.
- **Decoupled memory growth:** The chars and fields arrays in this model grow at different rates, making it harder to reason about situations where memory allocation is required.

### Model C

Use the ECMA-402 data model within the implementation.

```
parts = [
  { field: 'd', str: "19"},
  { field: '0', str: "/" },
  { field: 'm', str: "11" },
  { field: '0', str: "/" },
  { field: 'y', str: "2020" },
]
```

Pros:

- **Easy to implement:** Prepending and inserting strings is trivial.

Cons:

- **Harmful to the normal use case:** `formatToParts` is the exception, not the rule.  With this model, we either need extra computation to render the parts array to a string, or keep two separate string and parts structures in memory, which brings back the cons of Model B.

### Model D

Keep two parallel arrays, one with code points and the other with structured [BIES](https://www.researchgate.net/figure/Position-tags-in-a-word-BIES-tags-Tag-Description_tbl1_250723695) (begin, inside, end, single) annotations.

```
chars = "19/11/2020"
fields = [
  { field: 'd', bies: 'b' },
  { field: 'd', bies: 'e' },
  { field: '0', bies: 's' },
  { field: 'm', bies: 'b' },
  { field: 'm', bies: 'e' },
  { field: '0', bies: 's' },
  { field: 'y', bies: 'b' },
  { field: 'y', bies: 'i' },
  { field: 'y', bies: 'i' },
  { field: 'y', bies: 'e' },
]
```

This model can support multiple levels of field:

```
chars = "19/11/2020"
fields = [
  { lvl1: ['day',   'b'], lvl2: ['int',  'b'] },
  { lvl1: ['day',   'e'], lvl2: ['int',  'e'] },
  { lvl1: ['null',  's'], lvl2: ['null', 's'] },
  { lvl1: ['month', 'b'], lvl2: ['int',  'b'] },
  { lvl1: ['month', 'e'], lvl2: ['int',  'e'] },
  { lvl1: ['null',  's'], lvl2: ['null', 's'] },
  { lvl1: ['year',  'b'], lvl2: ['int',  'b'] },
  { lvl1: ['year',  'i'], lvl2: ['int',  'i'] },
  { lvl1: ['year',  'i'], lvl2: ['int',  'i'] },
  { lvl1: ['year',  'e'], lvl2: ['int',  'e'] },
]
```

Pros:

- **Efficient:** Same as Model A
- **Linear:** Same as Model A
- **Generalizable:** Unlike Model A, the structure can represent distinct adjacent fields, and nested fields can be represented by making smaller linear segments with more specific field types.

Cons:

- **Complex nested fields:** Although nested fields are supported, it can be complex and bug-prone to implement logic for them, although most of that complexity can be hidden away from the user. Care must be taken when implementing this approach.
- **Less intuitive:** Potentially less clear than the other options to ICU4X developers.

### Model E

Like Model B, but set the origin to a nonzero index corresponding to the position within the buffer (see Appendix A).

For example, suppose the buffer had length 20, and the string started at index 5. The following internal model could be used:

```
chars = "19/11/2020"
fields = [
  { field: 'd', start: 5, end: 7 },
  { field: '0', start: 7, end: 8 },
  { field: 'm', start: 8, end: 10 },
  { field: '0', start: 10, end: 11 },
  { field: 'y', start: 16, end: 15 },
]
```

Pros:

- **Generalizable:** Same as Model B
- **Fast on prepend and append:** Fixed from Model B

Cons:

- **Decoupled memory growth:** Same as Model B

## Appendix A: String Operations for i18n

When building a formatted string, there are three high-level operations:

1. Prepend: add a string to the beginning of the builder.
2. Insert: add a string in the middle of the builder.
3. Append: add a string to the end of the builder.

Prepend and append are both highly efficient operations on character buffers.  With either prepend or append, extra buffer space can be allocated before or after the string.  With prepend, the start pointer is moved forward to make room for the string at the beginning; with append, the end pointer is moved back.  With either prepend or append, when the buffer runs out of space, the string needs to be copied to a new, larger buffer; however, if the buffer size is doubled each time, this operation [amortizes to a constant factor](http://anh.cs.luc.edu/363/notes/06A_Amortizing.html).

Insert is less efficient than prepend and append, because it requires part of the string to be shifted either forward or backward to make room for the new string segment.  I therefore will not focus on the insert operation for the subsequent two lemmas.

Note: the same logic applies to the inverse operations: shift (remove from start), splice (remove from middle), and pop (remove from end).  Shift and pop are both highly efficient, and less so for splice.  Although these operations are less common when building formatted strings, they do have instances where they arise, such as certain operations seen in measurement unit formatting that require replacing part of a string with a different string.

### Lemma 1: Prepend is uniquely inefficient with Model B

Model B requires each individual field to retain pointers into the formatted string.  These pointers need to be updated whenever a prepend operation is performed.  This is a quadratic operation.

For example, consider:

- Current string: `"20"` with fields: `[{ field: "day", start: 0, end: 2 }]`
- String to prepend: `"November "` with field `"month"`

With the prepend operation, the string `"November "` is prepended to the start of the char buffer.  However, when adding the field information, not only do we need to add `{ field: "month", start: 0, end: 9 }` to the beginning of the field array, but we also need to update all elements following it to reflect the new 9 characters, such that we end up with `[{ field: "month", start: 0, end: 9 }, { field: "day", start: 9, end: 11 }]`.  This is a quadratic operation because as the string gets longer, in the limit, every existing field needs to be updated when a new substring is prepended, so if you prepend substrings N times, you need to perform O(N^2) updates.

To contrast, the prepend operation is efficient for fields in all of the other models:

- Model A: When prepending chars to the string buffer, prepend an equal number of chars to the field buffer.  Both buffers have the same length and therefore the same efficient behavior.
- Model C: Both the substring and the corresponding fields are prepended at the same time to the all-in-one parts buffer.
- Model D: Similar to Model A, a BIES segment of the same length as the new substring is prepended to the start of the fields buffer, which has the same length as the string buffer.

### Lemma 2: Prepend is a useful operation

My goal in this section is to illustrate situations where prepend may be a convenient option to ICU4X developers building formatters.  All of the examples in this section are based on my personal experience working with formatters in ICU4C and ICU4J.

#### Composing prefixes and suffixes in NumberFormat

NumberFormat is based on the concept of prefixes and suffices.  Compact notation and measurement unit patterns typically take the form "P{0}S", where "P" is the prefix, "S" is the suffix, and "{0}" is the rest of the number.  For example, in English, the pattern for more than one meter is "{0} meters".  In other languages, a prefix may be used instead of a suffix, and in some languages, both a prefix and a suffix are used.

When building a formatted number string, one reasonable approach is to build the string from the inside out.  First you put the mantissa in the string, and then you apply prefixes and suffixes one by one.  This requires a prepend operation for prefixes and an append operation for suffixes.

The reason building formatted number strings from the inside out is a reasonable approach is because the mantissa can affect the rendering of the prefix and/or suffix; for example, with UTS 35 currency spacing, where the presence of a space between the currency symbol and the mantissa depends on whether the mantissa contains a certain character.

#### Building a formatted list

For lists of 3 or more items, the list formatting algorithm in UTS 35 works like this: apply a pattern such as `"{0}, {1}"` for all but the final element, and then `"{0}, and {1}"` for the final element, where "{0}" represents the list up to that point, and "{1}" represents the next element.

In cases where the "{0}" appears at the very beginning and "{1}" appears at the very end of the string, assembling the formatted list is done using only append operations.  However, the UTS 35 spec allows for text to appear before the "{0}" or after the "{1}", and even for the "{0}" and "{1}" to switch places.  Examples of locales with such data are ur, fa, and ml.  Writing a ListFormatter to support those locales involves a prepend or insert operation.

### Conclusions

My goal with Lemma 2 is *not* to argue that prepend is the *only* option in these situations; most of these situations can be architected in a way that avoids the need for prepend.  What I'm trying to argue is that prepend is *useful*, and that Model B eliminates that option, *requiring* code to be architected in a way to exclusively use append and never to use prepend.
