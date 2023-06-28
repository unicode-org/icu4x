ICU4X Style Guide
=================

This document outlines the style guide and best practice for code in ICU4X, with a focus on Rust code style.

## Objectives

This style guide is intended to help ICU4X code be readable and maintainable for many years to come, as well as run quickly on devices of all sizes and operating systems with low memory usage and code/data size.

*All pull requests must fully abide by this style guide.* However, this is an evolving document; if you feel that a recommendation in this guide comes into conflict with correctness, readability, or performance, please suggest an update to the guide.

## Preamble

As Rust is a new programming language it is expected that the language guidelines will evolve over time, so care should be taken to keep this document up to date, and any new changes after the initially agreed upon version should be reflected in the change history at the bottom of this document.

Before reading this, it is recommended that you read most of [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html) and [Rust by Example](https://doc.rust-lang.org/rust-by-example/#rust-by-example). There's enough new concepts and syntax in Rust that it's not a language that's easy to pick up "as you go".

Items in this document are categorized as **required** or **suggested**, which essentially maps to the __MUST__ and __SHOULD__ language of Internet RFCs. Any use of the more lenient MAY is deliberately avoided here, since there's no need to encourage too much personal preference in this code base.

Many of the practices here are collected from existing sources (which should be cited) and where applicable there may be some additional justification for including that practice in this document and why it is categorized as it is. In general where a practice is sensible, expected to be achievable, and has no obvious downside, it will be marked as required.

If you find yourself in a situation where it would be better to violate a **required** practice, please raise it with the group so that we can have a discussion about it. There will almost certainly be many cases where we need to update this doc and even go back to existing code to update it in the light of new information. However, exceptions should always be commented clearly for the next maintainer.

If you're new to Rust, see the [Appendix](#appendix) for some, hopefully useful, links.

## Naming Conventions

### No Special Naming :: required

Follow the naming advice in [Naming - Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html) at all times, and use [Rust RFC 0430](https://github.com/rust-lang/rfcs/blob/master/text/0430-finalizing-naming-conventions.md) for any ambiguous issues (e.g. Crate naming).

* Infer Crate names from package names where feasible:
  * A package is the directory in which the Cargo.toml exists.
* Prefer single (lowercase) words for package names where feasible
* Use __lowercase-with-hyphen__ when multiple parts exist in a package name.
  * This seems to be the preferred Rust community approach.
* Hyphens in package names are converted to __underscore__ in Crate names.
  * To avoid ambiguity, do not use underscore in package names.

This should result in Crate names always being unambiguously mappable to/from package names (e.g. the Crate "foo_bar" relates to the package "foo-bar").

Each component should have its own unique name in form `icu-{package}` which will be used for its releases. When released as part of the meta `icu` package, it will be re-exported as `icu::{package}::{struct}` and for core structures it may also be re-exported as `icu::{struct}}`.

Within a single crate, exposure of structs and functions via their modules is up to the discretion of the author. As a rule of thumb, items that are required for common patterns should be available directly, while items needed only for a subset of use cases may be exposed via their modules.

In the example below, since date and time skeletons aren't used in most common scenarios, they are exposed via the `skeleton` module.

### Variable naming :: suggested

General convention is to use snake_case naming for variables, see [Naming - Rust API Guidelines](https://rust-lang.github.io/api-guidelines/naming.html).

Variables used in loops, closures and in narrow scope (handful of lines) can be abbreviated to one or more characters, e.g.

`
vec![2, 3].into_iter().fold(0, |l, r| l + r)
`

In all other cases, esp. in public APIs, use non-abbreviated names, like locale vs loc, canonicalized_locale vs canon_loc, item_count vs item_ct.

### Naming Exported types :: suggested

Follow the naming advice in [Naming - Rust API
Guidelines](https://rust-lang.github.io/api-guidelines/naming.html) in general.

The ICU4X project has adopted some internal conventions for naming exported
types.  Note that the intention of the conventions is to have common guidelines
that make ICU4X code look uniform.  It is not the intention to prescribe the
one true way. As the discussion around naming in the broader Rust community is [still
ongoing][namingd], we will use some conventions that have been popular in the
current Rust code bases.

[namingd]: https://internals.rust-lang.org/t/does-the-avoid-prefixing-the-type-with-the-name-of-the-module-style-rule-still-apply/12819

#### Exported Type naming

This concerns types that are available for public use outside of Rust.

For similarly named types that serve analogous purposes in orthogonal APIs,
prefer adopting slightly repetitive names as shown below:

* **DO**: `pluralrules::PluralRulesOptions` and `listformat::ListFormatOptions`.
* **DON'T**: `pluralrules::Options` and `listformat::Options`.
* **Rationale**: The ICU4X team has a preference for importing types by name and not
  using module names as disambiguators.  In the 'DO' case above, one would do:

  ```rust
  use {
    pluralrules::PluralRulesOptions,
	listformat::ListFormatOptions,
  }

  // ... later
  let opt1 = PluralRulesOptions::new();  // vs pluralrules::Options::new()
  let opt2 = ListFormatOptions::new();  // vx listofrmat::Options::new()
  ```

* **DO**: `pluralrules::PluralRulesError`
* **DON'T**: `pluralrules::Error`
* **Rationale**: In IDE and rustdoc, only an unqualified name  (`Error`)  appears,
  leading to confusion about what the fully qualified error type is.

#### Enums

You can omit repetition in enum variants, as enums are usually qualified with
their type.

* **DO**:

  ```rust
  pub enum SomeError {
      Argument,
      InvalidData,
  }
  ```
* **DON'T**:

  ```rust
  pub enum SomeError {
      ArgumentError,
      InvalidDataError,
  }
  ```
* **Rationale**: In this case, as we will likely refer to the enum value by
  `use SomeError` and then `SomeError::Argument`, additional repetition in the
  name doesn't seem needed.

#### Examples

| Package | Crate | Standalone Import | ICU Meta-package |
|----|----|----|----|
| locale | `icu_locid` | `use icu_locid::Locale` | `use icu::Locale` |
| plurals | `icu_plurals` | `use icu_plurals::PluralRules` | `use icu::PluralRules` |
| datetime | `icu_datetime` | `use icu_datetime::DateTimeFormat` | `use icu::DateTimeFormat` |
| datetime | `icu_datetime` | `use icu_datetime::skeleton::SkeletonField` | `use icu::datetime::skeleton::SkeletonField` |

While the scheme may feel repetitive when looking at the import lines, it pays off in being unambiguous without aliasing when multiple structs from different components get used together:

```rust
use icu_locid::Locale;
use icu_datetime::{DateTimeFormat, DateTimeLength, skeleton::{Skeleton, SkeletonField}};
use icu_list::ListFormat;

fn format() -> Result {
    let loc: Locale = "ru".parse()?;

    let mut dt_ops = Default::default();
    dt_opts.date_length = DateTimeLength::Long;

    let dtf = DateTimeFormat::try_new(loc.clone(), dt_opts)?;

    let lf = ListFormat::try_new(loc, Default::default())?;

    assert_eq!(Skeleton::from_length(dt_opts.date_length).get(0), Some(SkeletonField::YYYY));
}
```

Note: See also [this discussion on API guidelines](https://github.com/rust-lang/api-guidelines/issues/29#issuecomment-342745898).

## Module and Code Layout

### Don't have "mixed" Crates :: suggested

The current thinking is that you should either:

* Use a single `lib.rs` file for a Crate with everything in it
* Use multiple Rust sources and then only use `lib.rs` to export modules

Which of these options you chose is dependent on the size/complexity of the module(s).

There might be cases where an intermediate "mixed" Crate is worthwhile, but it should be clearly documented as to why it is necessary.

### Avoid over-exposing internal APIs :: suggested

When sharing an API between modules in a Crate, use `pub(<scope>)` to express visibility, and select the smallest scope which suits the intent of the API (i.e. prefer `pub(crate::specific_mod)` to `pub(crate)` where appropriate).

## Code Formatting and Linting

### Use "cargo fmt" prior to any code reviews :: required

Hopefully self explanatory. The less time we spend worrying about formatting and the fewer diffs during code reviews, the better.

### Run "cargo clippy" and accept its suggestions :: suggested

There are bound to be some cases where the linter makes suggestion we don't want to follow, but these should be rare. Any such false-positives should be [suppressed](https://github.com/rust-lang/rust-clippy#allowingdenying-lints) and commented for future maintainers.

GitHub CI enforces ICU4X's clippy standards.

### Render visible characters in docs and code :: suggested

Often in internationalization, we deal with code points from other scripts. It is suggested to put those code points directly into the file, _instead of_ using escape syntax such as `\u{1234}`. Exceptions may include:

- If directly rendering the characters could cause confusion due to the bidi algorithm
- If the test cares more about the code point values than the characters they represent

### Render invisible characters in docs but escape them in code :: suggested

There are several types of invisible code points in Unicode, including whitespace, zero-width characters, and bidi marks. The policy surrounding these characters is:

- In docs tests: users are more interested in the end result, so favor rendering the invisible characters. When invisible characters could cause confusion, it is suggested to leave an unrendered docs comment, such as `# // The following line contains an invisible code point.`
- In source code and unit tests: being explicit about invisible characters makes reading and modifying code more explicit for ICU4X developers, so favor using escape sequences.

# Structs and Traits

For ICU4X we should be looking to keep code consistent and very unsurprising, especially for layout and behavior of data structures.

## Private vs Public

Rust offers a compelling model for encapsulating implementation details based on a hierarchical view of privacy; see [Visibility and Privacy - The Rust Reference](https://doc.rust-lang.org/reference/visibility-and-privacy.html). This should allow ICU4X to encapsulate all of its implementation details neatly from applications using the library and avoid too many cases of [Hyrum's Law](https://www.hyrumslaw.com/).

### Minimize user-visible structs :: suggested

While this sounds a bit obvious, it's important to stress that with Rust's privacy model, there is never any need to have an internal type be user-visible, and once a type is user-visible outside a Crate, it has a high cost associated with changing it (i.e. a semantic version change to the library). If there are parts of the API which must be released, but for which we cannot provide stability guarantees, they must be marked as such.

There is a known pattern of using [`doc(hidden)`](https://doc.rust-lang.org/rustdoc/the-doc-attribute.html#dochidden) and having module names prefixed with multiple underscores to indicate they must not be relied upon outside the project, but ideally APIs would be designed to minimize the need for this and the expected external usage would be clearly documented for future maintainers.

There are three areas which may warrant a technically public API that we don't consider stable:
* Where another crate may need access to non-public API
* Where we want to expose some non-public API for testing or benchmarking
* Where we want to expose some non-public API for helper macros

### Public fields vs getter/setter :: required

Each exposure of a public field should be considered carefully.
Public fields represent an equally binding API contract to a pair of getter/setter public methods without any validation.

There is no difference in field mutability between exposing them vs wrapping in a getter/setter model.
This is a major difference for users coming from the C++ family of languages.

Since mutability is controlled via ownership and borrow checker rules, if the getter is just returning a reference, and
the setter is only assigning the value, then exposing a public field is recommended for ergonomics.

#### Example

```rust
enum DateTimeLength {
    Long,
    Short,
}

struct DateTimeOptions {
    pub time_length: DateTimeLength,
    pub date_length: DateTimeLength,
}

fn main() {
    let mut opts = DateTimeOptions {
        time_length: DateTimeLength::Short,
        date_length: DateTimeLength::Long,
    }

    // Override
    opts.time_length = DateTimeLength::Long;
}
```

Since the `date_length` and `time_length` fields can accept only valid values, and the getter/setter methods
would not perform any additional operations, those fields can be exposed as public.

In all other situations, where the getter/setter are fallible, perform additional computations or optimizations, or an invariant between the fields has to be achieved, it is advised to use an equivalent getter/setter model:

```rust
#[derive(Default)]
#[non_exhaustive]
struct DateTimeOptions {
    meta: Option<TinyStr8>,
}

impl DateTimeOptions {
    pub fn get_meta(&self) -> Option<&str>;
    pub fn set_meta(&mut self, input: &str) -> Result;
}

fn main() {
    let mut opts = DateTimeOptions::default();

    // Override
    opts.set_meta("foo")?;
}
```

Note: Any change to field visibility constitute a breaking change to the public API.
Note: Even if the setter is infallible, the getter/setter model is useful when optimized type is used internally while public API exposes a standard type, like in the case of `meta` field in the example above.

One suggested situation in which public fields would be acceptable is for user-facing "bag of options" structs. These would have no inbuilt semantics and no consistency guarantees, so the superior ergonomics of the public fields can be benefited from. In that case, such struct should also implement or derive the `Default` trait. See `Constructor` section below for details.

See [this issue](https://github.com/unicode-org/rust-discuss/issues/15) for more.

## Derived Traits

### Debug Trait on Public Types :: suggested

It is recommended that all public types used by ICU4X support (at least) the [Debug](https://doc.rust-lang.org/std/fmt/trait.Debug.html) trait. This adds a bit of additional code, and requires that all fields in the struct also have this trait (unless you add a non-derived custom implementation).

### Implement Clone and Copy only as needed :: suggested

Which types implement these traits should be carefully considered and documented.

* **Clone** is a general purpose duplication mechanism.
  * To derive this trait you must have all fields being Clone-able.
  * Types can also implement this trait manually and provide custom business logic to handle ownership etc.
  * While deriving or implementing Clone is normal for most data types, it may not be suitable for other types, especially those backed by system resources (e.g. data providers).
* **Copy** is for types which can always be safely duplicated (e.g by bitwise copying).
  * A type which implements **Copy always implicitly implements Clone**.
  * You cannot override and customize how Copy is implemented.
  * For example, note that `String` does not implement Copy.

There are pros and cons for using these derived traits, but once a (user) public struct uses a derived trait, it is **part of the published API and cannot be removed**.

Simple value types will benefit the most from implementing Clone and Copy traits, since it allows users to avoid having to deal with lifetimes and borrowing. However this can prohibit (or at least complicate) the act of adding a new field to the struct (e.g. adding a `String` to a Copy-able struct).

Adding a trait later is very unlikely to be a breaking change (technically it is, but only in cases where someone has extended the type with module specific traits).

My guess is that we should prefer implementing Clone on user visible types (for convenience) but avoid promising Copy in most situations. I believe that the Rust compiler is sensible enough to implement Clone exactly as if it were Copy if all fields are Copy-able, but by not advertising this we get the **freedom to add non-copyable fields later without breaking anyone**.

This could be slightly annoying for users who want to put ICU4X types into otherwise Copy-able structs, so we might face pressure to make some common data types Copy-able.

Note: A type which has **neither Copy or Clone** can still be passed by value or returned from a function, but only when that operation can be determined to always be a "move" of the data (i.e. where only one instance is needed). Copy and Clone are needed when duplicate instances need to be created.

### Implement Copy on Error types and log details :: suggested

An "error type" is an enum or struct that is usually returned in the `Err` variant of a `Result`.

In ICU4X, we have a convention of making error types implement `Copy`. This is primarily because highly complex error types carry a code size burden: the `Drop` impls of such types are complex and significantly impact on binary size.

To associate additional metadata with errors, such as file paths, use the `log!` macro.

### Implement Eq / PartialEq, Ord / PartialOrd only as needed :: suggested

Which types implement these traits should be carefully considered and documented.

* [**Eq**](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [**PartialEq**](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html) are traits designed for comparing equality of types.
  * These traits define the behaviour of the `==`, `!=` operators.
* **PartialEq** is useful when a looser equality is required (e.g. comparing `NaN`s for floating point).
* [**Ord**](https://doc.rust-lang.org/std/cmp/trait.Ord.html) and [**PartialOrd**](https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html) are traits designed for ordering types.
  * These traits define the behaviour of the comparison operators (`<`, `>`, `<=`, `>=`).
  * Their behavior must be consistent with **Eq** and **PartialEq**.

Implementing comparison traits can be problematic and should only be done for types where it is semantically meaningful. Note that Rust is very opinionated on the expected behaviour of these traits, so (for example) you must not implement a non-symmetric equality operator or a non-transitive partial ordering.

One drawback to deriving the Eq trait "too early" would be if you later add some other non-Eq type to the struct. For example, **floating point types implement PartialEq but not Eq**.

### No "Clever" Comparisons :: suggested

The above comparison traits can be implemented for "interesting" type combination, not just on a single type. This would allow for semantically complex comparisons between user types (e.g. a mutual ordering between region codes and language codes). However this sort of API design, with implicit relations, is often subtle and easy to misuse. As the documentation for PartialEq states:

> Implementations of PartialEq, PartialOrd, and Ord must agree with each other. **It's easy to accidentally make them disagree by deriving some of the traits and manually implementing others**.

Note that we might choose to have non-derived orderings for some types (e.g. script sub-tags), but these should always be carefully documented.

## Sized Types

**Open Question**: Decide what to recommend here (if anything).

Types in Rust can be **sized** or **unsized**. A sized type knows at compile time how much space is needed to hold it, and can thus have stack space allocated or be copied into a Vec or array. This improves memory locality and reduces the need for heap allocations.

Perhaps more importantly, **an unsized type can only be passed by reference to other functions** and cannot be Copy-able (though they can be Clone-able). In particular any trait is, by default, unsized which makes storing traits in other structs a bit interesting (there is a [ToOwned](https://doc.rust-lang.org/std/borrow/trait.ToOwned.html) trait to help with this though).

This adds weight to the idea that we should avoid traits and any other unsized types when specifying user visible types. Obviously we will accept things like `str` as parameters (these are unsized), but we aren't adding them directly to structures by reference.

For example, in general we should avoid returning an abstract trait to the user (intermediate traits like Iterator might be fine though since a user is expected to consume those fairly immediately).

# Idiomatic Code

A lot of this section just boils down to "[Read the Book](https://doc.rust-lang.org/book/)", but I'm highlighting a few things I personally found useful while learning.

## Pass by Reference vs Pass by Value

There is something a bit subtle about how Rust handles "pass by value" which means you should not just apply standard C++ best-practice. In particular, in C++ you would might expect a method like:

```cpp
ReturnType ClassName::FunctionName(ParamType param) { … }
```

to copy the parameter and return types during the course of calling the method (e.g. as a memcpy to the stack). This leads C++ developers to favour passing objects by reference and passing pointers to hold return types, instead of taking the hit of copying the value.

Rust treats "pass by value" a little differently, and it can be thought more of an indication that "ownership is being transferred" rather than the API wants "a copy of the data". Thus the Rust equivalent code:

```rust
fn function_name(param: ParamType) -> ReturnType { … }
```

may still have parameters "passed by reference" when it can determine that the reference is no longer subsequently used by the calling code (this is a "move" in Rust parlance).

Furthermore, it will often [ellide the copy](https://en.wikipedia.org/wiki/Copy_elision) of the return value if it determines the returned object would otherwise go out of scope. It will allocate the space for the return value on the caller's stack or use the memory in a destination struct, to directly write the "returned" value in its final destination with no copying whatsoever. This is called Return Value Optimization (RVO) and while it is now available in C++ as well, it's a relatively new feature there.

It is still often better (for reasons of borrowing and ownership) to pass structs by non-mutable reference, but returning **newly created** results by value (even potentially large structures) is not expected to cause performance issues.

### Return new instance by value :: suggested

Use "return by value" for any functions which produce new data and let Rust handle optimizing away any copying, unless there's really measurable performance issue.

### Pass struct parameters by reference where possible :: suggested

When passing struct types to functions, it's always semantically safe to pass a reference (since there can be no race conditions around its use and it cannot be modified unexpectedly while being processed). The called function should be responsible for taking a copy if it needs to (e.g. by cloning or via the ToOwned trait).

### Avoid implicit allocations :: suggested

Where possible, the public API should not take a reference, if the value is going to be cloned internally.


#### Examples

```rust
struct ListFormat {
    locale: Locale
}

impl ListFormat {
    // BAD
    pub fn try_new(locale: &Locale) -> Result<Self, ()> {
      Ok(Self {
        locale: locale.clone()
      })
    }

    // GOOD
    pub fn try_new(locale: Locale) -> Result<Self, ()> {
      Ok(Self {
        locale
      })
    }
}
```

This rule may become a tradeoff in rare cases of APIs which optionally allocate. For example, if an API only needs a reference in 95% of cases, but in the remaining 5% needs to allocate, it should be explicitly marked as such.
An example of such an API in the `stdlib` is `HashSet::get_or_insert` and `HashSet::get_or_insert_owned`:

```rust
impl HashSet {
    pub fn get_or_insert(&mut self, value: T) -> &T;

    pub fn get_or_insert_owned<Q: ?Sized>(&mut self, value: &Q) -> &T where
        T: Borrow<Q>,
        Q: Hash + Eq + ToOwned<Owned = T>;
}
```

In this example, the `get_or_insert` method accepts an owned value, in order to use it if the set does not contain it already.
The `get_or_insert_owned` method allows the user to pass a reference to a value that implements `ToOwned`, and
this trait will be used in the scenario of allocation.

### Pass Option<T> by value where possible :: suggested

[Option](https://doc.rust-lang.org/std/option/enum.Option.html) is a bit special, and Rust goes to great lengths to avoid needing to allocate an additional instance for this type. In particular, `Option<char>` is a zero-allocation representation and should be passed by value. This applies to some types where the extra [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) state can be encoded as an otherwise "invalid" bit pattern.

In the case of `Option<char>`, `None` is [encoded as `0x110000`](https://rust.godbolt.org/z/-ZFwKB) or other invalid bit patterns. This means types like `Vec<Option<char>>` or `[Option<char>;N]` should be preferred over `Vec<&Option<char>>` or `[&Option<char>;N]` which will often remove the need for additional lifetimes.

There are times where this may not be possible (e.g. if using non-`Copy` types you may need `&Option<T>` or `Option<&T>`), and there's a trade off between designing `Copy` types, exppected to be smaller value types, and other more complex types.

### Pass Box<T> by value where possible :: suggested

It's unlikely we will have many `Box<T>`s in public APIs. We should return boxes by-value, and accept things as `&T` as much as possible, instead of `&Box<T>`. Internally we might use `Box<T>` for efficiency, but it should be avoided in any public APIs.

### Pass and return fundamental type by value where possible :: required

Fundamental types are essentially free to pass around (at least never more expensive than passing a pointer) so unless you are using a mutable reference to the location of a fundamental type (e.g. in-place modification in a `Vec`) there's no reason not to just pass the value itself directly. Passing them by value, will often simplify cases where they are manipulated or compared with constants.

## Option

### Use Option exclusively to represent "missing" data :: required

[Option](https://doc.rust-lang.org/std/option/enum.Option.html) is Rust's only recommended way to represent the equivalent of a "null" value (i.e. missing data). Option has a host of useful traits and methods which make it easy to manipulate, propagate and transform.

```rust
// Get the value or a default.
let x = opt_value.unwrap_or(other_value);
```

```rust
// Get the value or return an Err
// (this can only be called in a function that returns a Result)
let y = opt_value.ok_or("Error message")?;
```

```rust
// Transform with lazy default value (note how "into()" can elide the
// type since it's already known to be OtherType).
let z = opt_value.map_or_else(OtherType::get_default, |v| v.into());
```

## Iteration

### Strongly prefer functional iteration :: suggested

Prefer using iterators rather than directly indexing data to avoid the need for any explicit bounds checks. The Rust compiler will make this at least as performant as a manual loop.

```rust
// GOOD
let tags: Vec<Tag> = ["en", "fr", "de"].iter().map(Tag::from).collect();

// BAD
let mut tags: Vec<String> = Vec::new();
let lang = ["en", "fr", "de"];
for n in 0 .. lang.len() {
    tags.push(Tag::from(lang[n]));
}
```

## Enums

### Strongly prefer enums to define bounded sets of states :: suggested

Enums in Rust are cheap/free, and incredibly useful. They can be used (as in C++/Java) for providing named, bounded "choices", including avoiding bare boolean parameters, but they can also provide the basis for type-safe patterns such as [elegant finite state machines](https://bluejekyll.github.io/blog/fsm/rust/2015/08/13/rust-and-the-most-elegant-fsm.html) using generified enums with data.

It's probably worth noting here that the [Result](https://doc.rust-lang.org/std/result/) type itself is just a normal enum in Rust with two values (`Ok` and `Err`).

### Don't use enums to represent unbounded sets :: suggested

If a set of entities is unbounded or grows over time, use an identifier instead of an enum. For more details, see [enums_or_ids.md](../design/enums_or_ids.md).

### Don't use explicit usize values :: suggested

ICU4C has a convention of assigning stable integer values to enum entries. However, this is not common practice in Rust (main issue: [#115](https://github.com/unicode-org/icu4x/issues/115)). Instead, limit the definitions of stable values to the FFI layer.

## Matching

### Prefer match statements for exhaustive conditional code :: suggested

Matching is Rust's idiomatic way to handle any non-trivial exhaustive conditional code, and works elegantly with enums, but handles any data for which a predicate can be formed (which is basically everything).

Even a simple if/else block can be expressed more idiomatically using a match statement and will produce essentially the same compiled code after optimization. This is especially true if you want all cases to be covered, since this is enforced by a match statement.

From [Rust By Example](https://doc.rust-lang.org/rust-by-example/flow_control/match.html):

```rust
match number {
    // Match a single value
    1 => println!("One!"),
    // Match several values
    2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
    // Match an inclusive range
    13..=19 => println!("A teen"),
    // Handle the rest of cases
    _ => println!("Ain't special"),
}
```

Another nice property of match is that it returns a value, which tied to the fact that it must cover all cases means you get a readable idiom for ad-hoc matching and mapping:

```rust
let x = match number {
    0     => "Zero",
    1..=9 => "Some",
    _     => "Many",
}
```

Obviously where an if-statement is simply there to do optional work, and not cover every case, it may well be more suitable to just use that.

# Structs

## Structs with Private Fields

### Constructor conventions :: suggested

If the struct doesn't require any arguments to be initialized, it should implement or derive the `Default` trait.
For structs with argumented constructors, `new` or `try_new` methods should be used for canonical construction, `From` and `TryFrom` traits should be implemented for generic conversions and `from_*` and `try_from_*` for non-trait based specific conversions.

| Type | Fallible | Trait | Use |
| ---- | ---- | ---- | ---- |
| Default | 𐄂 | `Default` | `Struct::default();` |
| Arguments | 𐄂 | | `Struct::new(locale, options);` |
| Arguments | ✓ | | `Struct::try_new(locale, options)?;` |
| `&str` | ✓ | `FromStr` | `value.parse()?` |
| Type | 𐄂 | `From` | `Struct::from(value);` |
| Type | ✓ | `TryFrom` | `Struct::try_from(value)?;` |
| Other | 𐄂 | | `Struct::from_{name}(value);` |
| Other | ✓ | | `Struct::try_from_{name}(value)?;` |

## Options structs With All Public Fields

Many ICU related constructors require a number of options to be passed. In such cases, it is recommended to provide a separate structure that is used to assemble all required options, and pass it to the constructor.

If mutability is needed, one can always add `Struct::extend_with(&mut self);` method or a constructor which takes a previous instance and constructs a new instance based on the old one and additional data.

### Implement `Default` and `#[non_exhaustive]` :: required

All such structs should also implement `Default` trait with `#[non_exhaustive]` attribute to simplify common construction models:

```rust
fn main() {
    let s = MyStruct::try_new(locale, MyStructOptions::default()).expect("Construction failed.");
}
```

This model provides a good separation between the `options` struct which most likely will be mutable while used, and the final struct which can be optimized to only contain the final set of computed fields and remain immutable.

The `#[non_exhaustive]` attribute disabled users ability to construct the Options struct manually, which enables us to extend the struct with additional features without breaking changes.

See the [Exhaustiveness](#exhaustiveness--required) section for more details.

### Examples

```rust
#[derive(Default)]
#[non_exhaustive]
struct MyStructOptions {
    pub min_fraction_digits: usize,
    pub max_fraction_digits: usize,
}

impl Default for MyStructOptions {
    pub fn default() -> Self {
        Self {
            min_fraction_digits: 3,
            max_fraction_digits: 5,
        }
    }
}

impl MyStructOptions {
   // Additional helper methods for setting the options,
   // or validating the consistency of the options.
}

struct MyStruct {
  locale: Locale,
  fraction_digits: Range<usize>,
}

impl MyStruct {
    pub fn new(locale: Locale, options: MyStructOptions) -> Result<Self, ()> { ... };
}

fn main() {
    let mut options = MyStructOptions::default();
    options_max_fraction_digits = 10;
    // Optional debug time validation of the options
    debug_assert!(options.validate());

    let s = MyStruct::try_new(locale, options).expect("Construction failed.");
}
```

## Data Types

### Zero-copy in DataProvider structs :: required

All data structs that can be passed through the DataProvider pipeline must support *zero-copy deserialization:* in practice, no heap allocations should be required when deserializing from Bincode-like formats. This means that if the type involves variable-length data like strings, vectors, and maps, it must use a zero-copy type backed by a byte buffer to represent them.

Data structs with zero-copy data should have a `'data` lifetime parameter.

In order to enable zero-copy deserialization via Serde, the `#[serde(borrow)]` annotation is most likely required. However, be aware of [known bugs](https://github.com/serde-rs/serde/issues/2016) regarding `#[serde(borrow)]` with `Option` types.

Examples of types that can be used in zero-copy data structs:

- Strings: `Cow<'data, str>`, except as noted below
- Vectors of fixed-width types: `ZeroVec<'data, T>`
    - Examples: `ZeroVec<'data, u32>`, `ZeroVec<'data, TinyStr8>`
- Vectors of variable-width types: `VarZeroVec<'data, T>`
    - Example: `VarZeroVec<'data, String>`
- Maps: `ZeroMap<'data, K, V>`
    - Example: `ZeroMap<'data, TinyStr4, String>`

In addition to supporting zero-copy deserialization, data structs should also support being fully owned (`'static`). For example, `&str` or `&T` require that the data be borrowed from somewhere, and so cannot be used in a data struct. `Cow` and all the other types listed above support the optional ownership model.

### Conventions for strings in structs :: suggested

Main issue: [#113](https://github.com/unicode-org/icu4x/issues/113), [#571](https://github.com/unicode-org/icu4x/issues/571)

When structs with public fields contain strings, use the following type conventions:

- `Cow<'data, str>` for data provider structs (those with `'data`).
- `String` for source data structs (with no lifetime).
- `&str` if the struct does not need to own the string.
- [TinyStr](https://github.com/zbraniecki/tinystr) if the string is ASCII-only.

### Pre-validation of options :: suggested

Main issue: [#158](https://github.com/unicode-org/icu4x/issues/158)

When a variable or struct field needs to adhere to certain invariants, such as a currency code being 3 letters or significant digits being between 0 and 20, use a type that can only represent valid option values.

```rust
// BAD
#[derive(Default)]
#[non_exhaustive]
struct MyStructOptions {
    /// Must be between 0 and 20
    pub fraction_digits: usize,
}

// GOOD
#[derive(Default)]
struct FractionDigits(usize);
enum Error {
    OutOfBounds,
}
impl FractionDigits {
    fn try_new(value: usize) -> Result<Self, Error> {
        if value >= 0 && value <= 20 {
            Ok(Self(value))
        } else {
            Err(Error::OutOfBounds)
        }
    }
}
#[derive(Default)]
#[non_exhaustive]
struct MyStructOptions {
    pub fraction_digits: FractionDigits,
}
```

### Pre-parsed fields (exotic types) :: suggested

Main issue: [#523](https://github.com/unicode-org/icu4x/issues/523)

Data in memory should be fully parsed and ready to use. For example, if a data struct contains a datetime pattern, that pattern should be represented as a `Pattern`, not as a string. We call these *exotic types*.

Keep the following in mind when using exotic types:

1. **Stability:** Since exotic types become part of the serialization format of the data struct, their serialized form must remain stable, according to the data struct versioning requirements discussed in [data_pipeline.md](../design/data_pipeline.md).
2. **Zero-Copy:** If the exotic type involves variable-length data (like a string or a vector), it must also support zero-copy deserialization, as described above. This means that such an exotic type must have a lifetime parameter and internal `Cow`s or `ZeroVec`s for data storage.
3. **Patching:** The exotic type should support an owned (`'static`) mode to allow users to patch their own data into a data struct, as explained above.
4. **Data Integrity:** In most cases, it is insufficient to auto-derive `serde::Deserialize` on an exotic type. Deserialization must perform data validation in order to retain internal invariants of the exotic type.

If it is not possible to obey these requirements in an exotic type, use a standard type instead, but make sure that it requires minimal parsing and post-processing.

# Error Handling

See also the [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) chapter in the Rust Book.

The ICU4X library should be designed so as to never fail at runtime. Now obviously that's something you'd expect all library writers to say, but in Rust you can control the places where code can fail explicitly, so it's much easier to write self-documenting code where the assumptions around failure are obvious.

Most core rust APIs (traits) have two ways to access data, a version that can "panic" , and one which returns a [Result](https://doc.rust-lang.org/std/result/index.html) or an [Option](https://doc.rust-lang.org/std/option/enum.Option.html). Rust is a language which likes to avoid "unnecessary" overhead, and in some cases it is perfectly correct to use an API which can "panic" because you have already checked the arguments carefully (and performance will be better).

Note that in cases where the Rust compiler can statically determine that a check is sufficient to avoid panic, it will remove the internal check and panic related code, leaving just a provably safe data access.

### Where Result is needed, use IcuResult<T> :: required

While it's still an open question in the Rust community as to what the best way to handle error is, the current ICU4X consensus is that we should start simple and expect to revisit this topic again at some point. The simplest reasonable starting point would be to have a `IcuResult<T>`, which is type as `Result<T, IcuError>`, where:

```rust
// Nesting semantically interesting error information inside the generic error type.
enum IcuError {
    Parser(parser::ParserError),
    Runtime(...)
}
```

A couple of crates by `@dtolnay` and `@yaahc` that are considered "new wave of good error APIs" and are complementary to each other:

* https://github.com/dtolnay/thiserror
* https://docs.rs/eyre/0.6.5/eyre/

Other links on error handling:

* https://blog.yoshuawuyts.com/error-handling-survey/
* http://sled.rs/errors
* https://boats.gitlab.io/blog/post/failure-to-fehler/
* https://boats.gitlab.io/blog/post/why-ok-wrapping/
* https://vorner.github.io/2020/04/09/wrapping-mental-models.html
* https://yaah.dev/try-blocks

## Panicking APIs

The most common example of an API which can panic is access to slices and elements of a slice. This includes accessing `array`, `str`, `Vec`, `HashMap` etc. via the `[`,`]` (square bracket) operator, implemented by the [Index](https://doc.rust-lang.org/std/ops/trait.Index.html) trait.

Thus statements like:

```rust
let x = self.data[n];
```

are always prone to "panic" if the index/key is incorrect. (⚠️ and this includes accessing maps ⚠️).

This is because these APIs return a value (or value reference) which cannot be "null", so there is no way for them to signal failure via the return type.

## Non-Panicking APIs

The alternative to using direct data accessors which can panic is to use a method which can return **Option** or **Result**. In the case of collections and strings, where a simple data item is being requested, this is most often provided by functions such as "get" (or "get_mut" for mutable references) which return `Option`.

If data access is expected to fail occasionally (e.g. looking up properties in a map) then the resulting [Option can be unwrapped](https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or) or propagated accordingly.

If missing data signals a "hard" error from which the function cannot recover (e.g. user supplies incorrect input) then any returned `Option` should be [propagated into a `Result` immediately](https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or), with an appropriate error value.

## Best Practice

### Don't Panic :: required

Call non-panicking data access APIs whenever data is not guaranteed to be safe.

This should not include the contract of code in a different Crate. I.e. if a function in a different Crate promises to return a valid map key, but it's not a compile time checked type (like an enum), then the calling code must allow for it to fail.

See also: the [Panics](#Panics--required) section of this document.

#### Special Case: `split_at`

The standard library functions such as `slice::split_at` are panicky, but they are not covered by our Clippy lints.

Instead of using `slice::split_at` directly, it is recommended to use a helper function, which can be saved in the file in which it is being used. Example:

```rust
/// Like slice::split_at but returns an Option instead of panicking
#[inline]
fn debug_split_at(slice: &[u8], mid: usize) -> Option<(&[u8], &[u8])> {
    if mid > slice.len() {
        debug_assert!(false, "debug_split_at: index expected to be in range");
        None
    } else {
        // Note: We're trusting the compiler to inline this and remove the assertion
        // hiding on the top of slice::split_at: `assert(mid <= self.len())`
        Some(slice.split_at(mid))
    }
}
```

#### Exception: Poison

The `lock`, `read`, and `write` methods on `Mutex` and `RwLock` return `Result`s in case the lock got poisoned. This happens when the process holding the lock panics, so it is fine to panic when encountering a poison. For consistency we require poisons to be handled with `.expect("poison")`.

### Avoid `to_string` :: suggested

`to_string` delegates to the `Display` trait which is bad for code size as it pulls in a lot of formatting code.

There are three types on which the standard library has *specialized implementations* for `to_string` that do not go through formatting logic: `&str`, `String`, and `char`. It would generally be acceptable to use `to_string` on these types, however types might not always be obvious to a reader, and implicit types might change with surrounding code (such as from `&str` to `&&str`, which is *not specialized*), so use `str::to_owned`, `String::clone`, and `String::from(char)` for clarity.

Any other invocation of `to_string` should be carefully evaluated. It is usually better to work with values that implement `Display` and/or `Writeable` than to eagerly format and allocate strings.

When comparing items in unit tests or docs, prefer semantic equality over stringified equality.

### Don't Handle Errors :: suggested

Functions which can error for any reason must return a `Result`, and APIs should be designed such that you should not generally need to recover from an [Err](https://doc.rust-lang.org/std/result/enum.Result.html#variant.Err) internally (which should normally be immediately propagated up to the user by using the [`?` operator](https://doc.rust-lang.org/edition-guide/rust-2018/error-handling-and-panics/the-question-mark-operator-for-easier-error-handling.html)). I.e. don't generally write library code which recovers from its own "errors", since if it can be recovered from, then it wasn't an "error".

This approach should mean that error handling and the design of functions which can propagate errors is consistent everywhere. For non-error cases, where different types of result are possible, use a normal enum.

Since an `Err` in `Result` is more expressive than a `None` in `Option`, there may be cases in which it is appropriate to handle a recoverable error. For example, you may call a function with one set of inputs, and if that call fails, you attempt to call it with a second set of inputs, before propagating the error.

Finally, and fairly obviously, **never turn an error into a panic by unconditionally unwrapping the result in the library**.

### Comment Use of Panicking Calls :: required

Use panicking methods only when the input has been explicitly checked to be correct.

```rust
// Attribute keys are checked for validity during data loading by ...
let x = self.attribute_map[char_attribute.key];
```

If this check does not occur immediately before the data access (i.e. shortly before in the same function), comment clearly where it does occur.

For example, if indices obtained from ICU data are to be trusted for indexed access, the data itself must have been validated at some earlier time (e.g. via a checking pass during data loading or use of a trusted hash).

However, you should **never add a check purely in order to call a method which could otherwise panic**; in that situation you should always prefer to call the non-panicking equivalent and handle the Option or Result idiomatically.

### Use Result over Option for errors :: suggested

When creating functions which can fail to return a value:
* Use **IcuResult** for all errors, or any cases where a user facing message is needed.
* Use **Option** for data accessors where "no data available" is a valid response (i.e. it's not an error per se).
  * Especially in cases where we expect the caller to have a reasonable response to getting [None](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None).
* Use a different enum for non-error cases with multiple return types (which can't use `Option`).

Examples:
* Does file with this path exists? - Option.
* Is there an element with this key in the list? - Option
* Try to open a file - Result
* Try to parse a string into a valid Language Identifier - Result

### Test all error cases :: required

You should write unit tests that cover all code paths that can generate an error.

If you find a bit of error-handling code that is unreachable by a unit test, you should consider replacing that code with `unreachable!()`.

## Integer Overflow

### Do not have unchecked overflow :: required

Malformed user input should not be able to cause integer overflow inside ICU4X implementation code.  You should return an error result if the user's input is too big and may cause integer overflow.

#### Use appropriately-sized integer types

You don't need a `usize` to represent a decimal digit 0-9.  By using the smallest possible integer type, you can reason better about cases where overflow can or cannot occur.

#### Checked Arithmetic

In cases where a hard limit on input is not possible, you can use methods such as [checked_add](https://doc.rust-lang.org/std/primitive.usize.html#method.checked_add).

#### Bounds Testing

By default, the `+` operator in Rust will panic upon overflow in debug mode.  You should employ thorough testing of boundaries to ensure that your arithmetic is never able to overflow.

### Don't accept infinite iterators :: suggested

Consider using `enumerate` to avoid an infinite iterator causing integer overflow by checking that the index does not exceed a bound that you set on the input.

Please do not mind that the following example could be written with `fold` instead of `enumerate`.

```rust
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Error {
  Limit,
}

/// iter: must contain fewer than usize::MAX elements
pub fn count_zeros<T>(iter: T) -> Result<usize, Error>
where
  T: Iterator<Item=u8>
{
  let mut result: usize = 0;
  for (i, v) in iter.enumerate() {
    if i == std::usize::MAX {
      return Err(Error::Limit)
    }
    if v == 0 {
      result += 1;
    }
  }
  return Ok(result)
}
```

# Lints

Some lints should be enabled at the crate level for primary ICU4X crates. This guidance need not extend to utils.

## Exhaustiveness :: required

Crates should deny the `clippy::exhaustive_structs, clippy::exhaustive_enums` lints at the top-level so that our types default to being `#[non_exhaustive]`.

These kinds of types _must_ be `#[non_exhaustive]`:

 - Options structs
 - Options enums
 - Error enums
 
Provider structs and enums _must not_ be `#[non_exhaustive]`. The provider module should ideally have `#[allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]`. These are expected to be stable at the Postcard level. Exceptions _may_ be made in cases where the deserializer is written in a way that may accept future fields or variants.

Most public newtypes and marker types should also be allowed to be exhaustive.

Miscellaneous types with public fields may or may not be exhaustive based on need: if they are expected to be stable, they should be marked with an allow attribute and a comment explaining why. Otherwise, default to `#[non_exhaustive]`.

## Panics :: required

Crates should deny the `clippy::indexing_slicing, clippy::unwrap_used, clippy::expect_used, clippy::panic` lints at the top-level to greatly reduce the number of panicky call sites in our code.

In general, non-panicky APIs that return `Result`s or `Option`s should be preferred.

`#[allow()]`s may be added, in cases where:

 - The panic will only occur if fundamental invariants of the codebase are invalidated. This should be avoided as much as possible, however
 - The API is clearly documented to be a panicky one and is not transitively used in other non-documented-as-panicky APIs.
 - The panic would only occur due to an invalidation of an API that is checked _very nearby_.
 - The code is pure test code.

`#[allow()]`s should be documented with a comment.

## Debug :: required

Crates should deny the `missing_debug_implementations` lint at the top-level so that our types all have `Debug` implementations.

# Imports and Configurations

## Features

### Use no_std :: suggested

Main issues: [#77](https://github.com/unicode-org/icu4x/issues/77), [#151](https://github.com/unicode-org/icu4x/issues/151)

Most ICU4X code will not work in [no_std](https://rust-embedded.github.io/book/intro/no-std.html), since memory allocation is very often required to handle edge cases.  Even our most fundamental type, Locale, requires memory allocation.

However, when designing traits and interfaces, we should make them `no_std`-friendly, such that we can more easily expand in this direction more easily in the future.

### When to add crate [features][features] :: suggested

When adding enhancements to an ICU4X component, introduce features as a way for
the end user to control the code size of their compilation as follows:

1. If the enhancement adds a new crate dependency, it should be behind a
   feature.
2. If the enhancement contains code that is not considered best practice, for
   example if it is mainly used for debugging diagnostics or development, then
   it should be behind a feature.

[features]: https://doc.rust-lang.org/cargo/reference/features.html

## Dependencies

### Avoid heavy dependencies :: suggested

Code size is an important factor for portability.  One of the easiest ways to accidentally bloat your code size is to pull in a heavy dependency.

When possible, write your code in such a way as to reduce dependencies, especially dependencies on heavier libraries.  If you need to add a dependency, consider putting it behind a feature flag.

### Avoid `std::collections::HashMap` :: suggested

The standard library HashMap makes bloated binaries.  In order to reduce code size, consider one of these options:

1. If the keys are known ahead of time, use a `struct` or `enum_map`.
1. Otherwise, use a sorted vector.

If using a vector, please note that sorting algorithms are also bloated.  Better practice to reduce code size is to either perform sorting offline (e.g., by requiring that data returned by the data provider is already sorted), or to perform binary searches when inserting new elements.  Example:

```rust
fn insert_sorted<A>(vec: &mut Vec<A>, item: A) {
  if let Err(idx) = vec.binary_search(&item) {
    vec.insert(idx, item);
  }
}
```

# Advanced Features

## Operator Overloading

### No clever operators :: required

Other than the comparison operators defined by [**Eq**](https://doc.rust-lang.org/std/cmp/trait.Eq.html), [**Ord**](https://doc.rust-lang.org/std/cmp/trait.Ord.html) etc. there is no obvious benefit to overloading other operators (e.g. overloading `*` to do something clever via the [Mul](https://doc.rust-lang.org/std/ops/trait.Mul.html) trait). This could be relaxed if standard Rust idioms for using particular operator overloads exists, and is well understood in the Rust community.

See also [Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html).

## Binding Traits to Inbuilt Types

Rust has neat ways to provide users with simple type conversion. Imagine if you have a function taking (reference to) a semantic type:

```rust
fn use_locale_id(id: &LocaleId) { … }
```

But the caller could just write:

```rust
use_locale_id(&"en_GB".into());
```

Rust's lets you bind traits to existing system type (e.g. `str`) for use within a module. And importantly, it lets you expose a series to trait bindings that other people can opt into if they want.
By implementing the generic [`TryFrom<&str>`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) trait on `LocaleId` to convert from a string to a locale ID, we also get the [`TryInto<Foo>`](https://doc.rust-lang.org/std/convert/trait.TryInto.html) trait implied on `&str` for free.

```rust
impl TryFrom<&str> for LocaleId {
  fn try_from(s: &str) -> IcuResult<LocaleId> {
    ...
  }
}
```

Note that there's also the [FromStr](https://doc.rust-lang.org/beta/std/str/trait.FromStr.html) trait for things which are explicitly parseable from strings.

However you can do more than just conversion types, and the [Unicode Segmentation](https://crates.io/crates/unicode-segmentation) crate binds a trait with many functions onto `str` to allow users to write things like:

```rust
use unicode_segmentation::UnicodeSegmentation;

// A vector of individual graphemes (true => extended).
let graphemes = "a\r\nb🇷🇺🇸🇹".graphemes(true).collect::<Vec<&str>>();

// Print each word (according to Unicode).
let s = "The quick (\"brown\") fox can't jump 32.3 feet, right?";
for w in s.unicode_words() {
  println!("{:?}", w);
}
```

Thus we could provide one or more ICU4X traits bound to things like `str` to provide a low friction way to access the libraries (obvious questions like naming notwithstanding).

# Appendix

## Sources

* [Learn Rust](https://doc.rust-lang.org/)
  * The canonical source for Rust information, but it doesn't offer advice on all aspects of code design.
* [Introduction - Learning Rust With Entirely Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/)
  * This is brilliantly written and very educational about how you get into a Rust mindset.
* [Elegant Library APIs in Rust](https://deterministic.space/elegant-apis-in-rust.html)
  * This has a lot of good points and is well worth a read, but be warned that some of the details about implementation are somewhat out of date (2017). It has a video too.
* [Good Practices for Writing Rust Libraries](https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/)
  * Shorter and more focused on the act of coding the libraries, and less about API design. Also potentially out of date in places.
* [Strategies for Returning References in Rust](http://bryce.fisher-fleig.org/blog/strategies-for-returning-references-in-rust/index.html)
  * Though I don't believe we should be doing this, it's still an interesting read.

## Other Useful Links

* Write and run Rust snippets: https://play.rust-lang.org
  * You can save snippets in permantent links and incluce them as working examples in docs.
* Write code and see what it compiles to: https://rust.godbolt.org
  * Note that you need to enable compiler optimizations via `-C opt-level=3` if you are looking to meaningfully compare two code snippets.
