Enums or Identifiers
====================

An *entity* is a discrete item in a set. For example, English is an entity: it is a specific instance of a language.

There are two general ways to represent entities: *enumerations* and *identifiers*. An *enumeration* is a fixed list of entities; an *identifier* is a universally understood space of numbers or strings. For example, `"en"` is an identifier: it is a language code according to a standard published by ISO.

This design doc helps explain how we think about entities in ICU4X and when to use enums versus identifiers to represent them.

## Decision Tree

If you need to decide whether to use an enum or an identifier for a certain set of entities in ICU4X, consider following this decision tree:

**Question 1:** How are new entities added to the set?

- *Spontaneously:* Use identifiers.
- *By an authority, at least once every few years:* Use identifiers.
- *Set is fixed or grows rarely:* Continue to Question 2.

**Question 2:** How many entities are in the set?

- *More than 20 or Infinite:* Use identifiers.
- *20 or fewer:* Continue to Question 3.

**Question 3:** Are the entities inclusive of all peoples and cultures? Is there a larger set out there which is not currently represented?

- *No:* Use identifiers or an enum with a private-use variant.
- *Yes or not applicable:* Use either enums or identifiers.

## Enumerations

An *enumeration* is a fixed list of entities. In Rust, an enumeration looks like this:

```rust
enum FooEnum {
    Entity1,
    Entity2,
    Entity3,
}
```

### Examples

Examples of entities that are represented by enums:

1. [`Signum`](https://unicode-org.github.io/icu4x-docs/doc/fixed_decimal/signum/enum.Signum.html):
    - Set of 4 values, determined by ECMA-402
    - Unlikely to change over time
    - Does not directly relate to peoples and cultures
2. [`PluralCategory`](https://unicode-org.github.io/icu4x-docs/doc/icu_plurals/enum.PluralCategory.html):
    - Set of 6 values, determined by Unicode/CLDR
    - Could theoretically change, but has not grown for a long period of time
    - Correctly represents all languages supported by CLDR

### Non-exhaustive enums

It is possible in Rust to mark enums as `#[non_exhaustive]`, which allows additional entries to be added over time.

However, if there is a need to do this, then it means that the set of entities could grow over time. If this is the case, consider using identifiers instead.

## Identifiers

An *identifier* is a universally understood space of numbers or strings, where a particular number or string represents a particular entity.

In ICU4X, identifiers are often represented as [new types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) wrapping the underlying representation:

```rust
struct FooId(u8);
```

### Examples

Examples of entities that are represented by strings:

1. [`Language`](https://unicode-org.github.io/icu4x-docs/doc/icu_locid/subtags/struct.Language.html):
    - New languages are frequently added to CLDR
    - Thousands of possible language codes
2. [Time zones](https://unicode-org.github.io/icu4x-docs/doc/icu_datetime/date/trait.TimeZoneInput.html):
    - New time zones are frequently created by governments
    - No ceiling to the number of possible IANA time zones

## Performance Considerations

A common reason developers tend to gravitate toward enums are perceived performance benefits. However, from experience, and with tools in place in ICU4X, the performance of identifiers is often strongly competitive with the performance of enums, with differences in the noise.

When choosing identifier schemes, keep the following in mind:

1. Integer values are often the most efficient or compact, but they are not usually human-readable. If using integers, it is best to base them off an existing specification.
2. An excellent alternative is [tinystr](https://docs.rs/tinystr/0.4.10/tinystr/index.html), which lets you represent short string values as integers. [`TinyStr4`](https://docs.rs/tinystr/0.4.10/tinystr/struct.TinyStr4.html), for instance, represents up to 4 ASCII characters as a `u32`.
3. If you must use a variable-length string, consider using a Cow-like storage mechanism such that the identifier can be created zero-copy from data files.

For example, time zones are often represented using the IANA names, such as "America/Chicago". However, in an effort to avoid variable-length strings in low-level functions, we instead opt for the BCP-47 time zone tags, like "uschi", which fit in a `TinyStr8`.
