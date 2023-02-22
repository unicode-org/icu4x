Enums or Identifiers
====================

An *entity* is a discrete item in a set. For example, English is an entity: it is a specific instance of a language.

There are two general ways to represent entities: *enumerations* and *identifiers*. An *enumeration* is a fixed, finite list of entities. An *identifier* is a universally understood space of numbers or strings, which may or may not be finite. For example, the identifier `"en"` is a language code according to a standard published by ISO.

This design doc helps explain how we think about entities in ICU4X and when to use enums versus identifiers to represent them.

## Decision Tree

If you need to decide whether to use an enum or an identifier for a certain set of entities in ICU4X, consider the following questions:

1. Is the set **fixed**?
2. Is the set **comprehensive**? Is it inclusive of all peoples and cultures? Are there no additional entities that are not currently represented?
3. Are there **30 or fewer items** in the set?
4. Are the entities used within the context of a single program, and **not normally interchanged** between different programs?
5. Are entities in the set usually **compared** with one another, and rarely expressed in a standalone context?

If you answered *yes* to the above questions, then an enum is probably the right choice. If you answered *no* to the above questions, then an identifier is probably the right choice. If there is a mix of *yes* and *no*, you are in a gray area; continue reading this document for additional guidance.

## Enumerations

An *enumeration* is a fixed list of entities. In Rust, an enumeration looks like this:

```rust
enum FooEnum {
    Entity1,
    Entity2,
    Entity3,
}
```

### Advantages

- Enums are useful for large codebase maintenance. When an enum changes, engineers can identify and update all call sites.
- The compiler has many opportunities to optimize enums; for example, memory layout and match statements.

### Examples

Examples of entities that are represented by enums:

1. [`Signum`](https://unicode-org.github.io/icu4x/docs/fixed_decimal/signum/enum.Signum.html):
    - Set of 4 values, determined by ECMA-402
    - Unlikely to change over time
    - Does not directly relate to peoples and cultures
2. [`PluralCategory`](https://unicode-org.github.io/icu4x/docs/icu_plurals/enum.PluralCategory.html):
    - Set of 6 values, determined by Unicode/CLDR
    - Could theoretically change, but has not grown for a long period of time
    - Correctly represents all languages supported by CLDR

### Private-use variants

An enum can have a private-use variant:

```rust
enum FooEnum {
    Entity1,
    Entity2,
    Entity3,
    PrivateUse(TinyStr8),
}
```

An enum with a private-use variant bridges the extensibility of identifiers with the engineering convenience of enums.

### Explicit discriminants

Enum values can be given explicit discriminant values:

```rust
enum FooEnum {
    Entity1 = 1,
    Entity2 = 2,
    Entity3 = 3,
}
```

If an enum is used for interchange between programs, including over FFI, discriminants should be explicitly specified. However, if the enum is only used within the context of the current program (if it never leaves Rust), explicitly specifying the discriminants could limit the compiler's ability to optimize the enum.

### Non-exhaustive enums

It is possible in Rust to mark enums as `#[non_exhaustive]`, which allows additional entries to be added over time.

Often, we do this in ICU4X out of an abundance of caution so that we can change APIs without breaking client code, such as option bag values. However, in other contexts, the need to mark an enum as `#[non_exhaustive]` could indicate that the enum is not fixed, which is a signal that an enum might not be the correct choice. As always, weigh this against the other pros and cons of enums versus identifiers.

## Identifiers

An *identifier* is a universally understood space of numbers or strings, where a particular number or string represents a particular entity.

Identifiers are also discussed and defined in [Unicode Technical Standard #39](http://unicode.org/reports/tr39/):

> Identifiers ("IDs") are strings used in application contexts to refer to specific entities of certain significance in the given application. In a given application, an identifier will map to at most one specific entity.

In ICU4X, identifiers are often represented as [new types](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) wrapping the underlying representation:

```rust
struct FooId(u8);
```

Using a new type means that any instance of an identifier is guaranteed to be valid. Validation could happen either at compile time (e.g., `const` values or a proc macro) or at runtime (e.g., deserializing from a data file).

### Advantages

- Identifiers are highly compatible with information interchange: the entity automatically comes with a universally understood serialization format.
- Identifier schemes are robust to changes in the world, since new entities can be readily assigned an identifier.

### Examples

Examples of entities that are represented by strings:

1. [`Language`](https://unicode-org.github.io/icu4x/docs/icu_locid/subtags/struct.Language.html):
    - New languages are frequently added to CLDR
    - Thousands of possible language codes
2. [Time zones](https://unicode-org.github.io/icu4x/docs/icu_datetime/date/trait.TimeZoneInput.html):
    - New time zones are frequently created by governments
    - No ceiling to the number of possible IANA time zones

## Performance Considerations

A common reason developers tend to gravitate toward enums are perceived performance benefits. However, from experience, and with tools in place in ICU4X, the performance of identifiers is often strongly competitive with the performance of enums, with differences in the noise.

When choosing identifier schemes, keep the following in mind:

1. Integer values are often the most efficient or compact, but they are not usually human-readable. If using integers, it is best to base them off an existing specification.
2. An excellent alternative is [tinystr](https://docs.rs/tinystr/0.4.10/tinystr/index.html), which lets you represent short string values as integers. [`TinyStr4`](https://docs.rs/tinystr/0.4.10/tinystr/struct.TinyStr4.html), for instance, represents up to 4 ASCII characters as a `u32`.
3. If you must use a variable-length string, consider using a Cow-like storage mechanism such that the identifier can be created zero-copy from data files.

For example, time zones are often represented using the IANA names, such as "America/Chicago". However, in an effort to avoid variable-length strings in low-level functions, we instead opt for the BCP-47 time zone tags, like "uschi", which fit in a `TinyStr8`.
