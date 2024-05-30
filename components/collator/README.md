# icu_collator [![crates.io](https://img.shields.io/crates/v/icu_collator)](https://crates.io/crates/icu_collator)

<!-- cargo-rdme start -->

Comparing strings according to language-dependent conventions.

This module is published as its own crate ([`icu_collator`](https://docs.rs/icu_collator/latest/icu_collator/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
`Collator` is the main structure of the component. It accepts a set of arguments
which allow it to collect necessary data from the data provider, and once
instantiated, can be used to compare strings.

Refer to the ICU User Guide sections for Collation that give an
[introduction](https://unicode-org.github.io/icu/userguide/collation/) and explain
[basic concepts](https://unicode-org.github.io/icu/userguide/collation/concepts.html).

## Examples

As its most basic purpose, `Collator` offers locale-aware ordering:

```rust
use core::cmp::Ordering;
use icu::collator::*;
use icu::locale::locale;

let locale_es = locale!("es-u-co-trad").into();
let mut options = CollatorOptions::new();
options.strength = Some(Strength::Primary);
let collator_es: Collator = Collator::try_new(&locale_es, options).unwrap();

// "pollo" > "polvo" in traditional Spanish
assert_eq!(collator_es.compare("pollo", "polvo"), Ordering::Greater);

let locale_en = locale!("en").into();
let mut options = CollatorOptions::new();
options.strength = Some(Strength::Primary);
let collator_en: Collator = Collator::try_new(&locale_en, options).unwrap();

// "pollo" < "polvo" according to English rules
assert_eq!(collator_en.compare("pollo", "polvo"), Ordering::Less);
```

### Examples of `CollatorOptions`

The [`CollatorOptions`] struct configures specific custom behavior for the `Collator`.  See docs
for [`CollatorOptions`] for more details.  Some basic descriptions and examples are below.

### Strength

The degree of sensitivity in how to determine that strings are distinct.

```rust
use core::cmp::Ordering;
use icu::collator::*;

// Primary Level

let mut options_l1 = CollatorOptions::new();
options_l1.strength = Some(Strength::Primary);
let collator_l1: Collator =
    Collator::try_new(&Default::default(), options_l1).unwrap();

assert_eq!(collator_l1.compare("a", "b"), Ordering::Less); // primary
assert_eq!(collator_l1.compare("as", "às"), Ordering::Equal); // secondary
assert_eq!(collator_l1.compare("às", "at"), Ordering::Less);
assert_eq!(collator_l1.compare("ao", "Ao"), Ordering::Equal); // tertiary
assert_eq!(collator_l1.compare("Ao", "aò"), Ordering::Equal);
assert_eq!(collator_l1.compare("A", "Ⓐ"), Ordering::Equal);

// Secondary Level

let mut options_l2 = CollatorOptions::new();
options_l2.strength = Some(Strength::Secondary);
let collator_l2: Collator =
    Collator::try_new(&Default::default(), options_l2).unwrap();

assert_eq!(collator_l2.compare("a", "b"), Ordering::Less); // primary
assert_eq!(collator_l2.compare("as", "às"), Ordering::Less); // secondary
assert_eq!(collator_l2.compare("às", "at"), Ordering::Less);
assert_eq!(collator_l2.compare("ao", "Ao"), Ordering::Equal); // tertiary
assert_eq!(collator_l2.compare("Ao", "aò"), Ordering::Less);
assert_eq!(collator_l2.compare("A", "Ⓐ"), Ordering::Equal);

// Tertiary Level

let mut options_l3 = CollatorOptions::new();
options_l3.strength = Some(Strength::Tertiary);
let collator_l3: Collator =
    Collator::try_new(&Default::default(), options_l3).unwrap();

assert_eq!(collator_l3.compare("a", "b"), Ordering::Less); // primary
assert_eq!(collator_l3.compare("as", "às"), Ordering::Less); // secondary
assert_eq!(collator_l3.compare("às", "at"), Ordering::Less);
assert_eq!(collator_l3.compare("ao", "Ao"), Ordering::Less); // tertiary
assert_eq!(collator_l3.compare("Ao", "aò"), Ordering::Less);
assert_eq!(collator_l3.compare("A", "Ⓐ"), Ordering::Less);
```

### Alternate Handling

Allows alternate handling for certain customized collation orderings, including the option to
ignore the special handling for the strings of such customizations.  Specifically,
alternate handling is used to control the handling of the so-called **variable** characters in the
Unicode Collation Algorithm: whitespace, punctuation and symbols.

Note that `AlternateHandling::ShiftTrimmed` and `AlternateHandling::Blanked` are
unimplemented. The default is `AlternateHandling::NonIgnorable`, except
for Thai, whose default is `AlternateHandling::Shifted`.

```rust
use core::cmp::Ordering;
use icu::collator::*;

// If alternate handling is set to `NonIgnorable`, then differences among
// these characters are of the same importance as differences among letters.

let mut options_3n = CollatorOptions::new();
options_3n.strength = Some(Strength::Tertiary);
options_3n.alternate_handling = Some(AlternateHandling::NonIgnorable);
let collator_3n: Collator =
    Collator::try_new(&Default::default(), options_3n).unwrap();

assert_eq!(collator_3n.compare("di Silva", "Di Silva"), Ordering::Less);
assert_eq!(collator_3n.compare("Di Silva", "diSilva"), Ordering::Less);
assert_eq!(collator_3n.compare("diSilva", "U.S.A."), Ordering::Less);
assert_eq!(collator_3n.compare("U.S.A.", "USA"), Ordering::Less);

// If alternate handling is set to `Shifted`, then these characters are of only minor
// importance. The Shifted value is often used in combination with Strength
// set to Quaternary.

let mut options_3s = CollatorOptions::new();
options_3s.strength = Some(Strength::Tertiary);
options_3s.alternate_handling = Some(AlternateHandling::Shifted);
let collator_3s: Collator =
    Collator::try_new(&Default::default(), options_3s).unwrap();

assert_eq!(collator_3s.compare("di Silva", "diSilva"), Ordering::Equal);
assert_eq!(collator_3s.compare("diSilva", "Di Silva"), Ordering::Less);
assert_eq!(collator_3s.compare("Di Silva", "U.S.A."), Ordering::Less);
assert_eq!(collator_3s.compare("U.S.A.", "USA"), Ordering::Equal);

let mut options_4s = CollatorOptions::new();
options_4s.strength = Some(Strength::Quaternary);
options_4s.alternate_handling = Some(AlternateHandling::Shifted);
let collator_4s: Collator =
    Collator::try_new(&Default::default(), options_4s).unwrap();

assert_eq!(collator_4s.compare("di Silva", "diSilva"), Ordering::Less);
assert_eq!(collator_4s.compare("diSilva", "Di Silva"), Ordering::Less);
assert_eq!(collator_4s.compare("Di Silva", "U.S.A."), Ordering::Less);
assert_eq!(collator_4s.compare("U.S.A.", "USA"), Ordering::Less);
```

### Case Level

Whether to distinguish case in sorting, even for sorting levels higher than tertiary,
without having to use tertiary level just to enable case level differences.

```rust
use core::cmp::Ordering;
use icu::collator::*;

// Primary

let mut options = CollatorOptions::new();
options.strength = Some(Strength::Primary);
options.case_level = Some(CaseLevel::Off);
let primary =
  Collator::try_new(&Default::default(),
                    options).unwrap();

assert_eq!(primary.compare("ⓓⓔⓐⓛ", "DEAL"), Ordering::Equal);
assert_eq!(primary.compare("dejavu", "dejAvu"), Ordering::Equal);
assert_eq!(primary.compare("dejavu", "déjavu"), Ordering::Equal);

// Primary with case level on

options.strength = Some(Strength::Primary);
options.case_level = Some(CaseLevel::On);
let primary_and_case =
  Collator::try_new(&Default::default(),
                    options).unwrap();

assert_eq!(primary_and_case.compare("ⓓⓔⓐⓛ", "DEAL"), Ordering::Less);
assert_eq!(primary_and_case.compare("dejavu", "dejAvu"), Ordering::Less);
assert_eq!(primary_and_case.compare("dejavu", "déjavu"), Ordering::Equal);

// Secondary with case level on

options.strength = Some(Strength::Secondary);
options.case_level = Some(CaseLevel::On);
let secondary_and_case =
  Collator::try_new(&Default::default(),
                    options).unwrap();

assert_eq!(secondary_and_case.compare("ⓓⓔⓐⓛ", "DEAL"), Ordering::Less);
assert_eq!(secondary_and_case.compare("dejavu", "dejAvu"), Ordering::Less);
assert_eq!(secondary_and_case.compare("dejavu", "déjavu"), Ordering::Less);  // secondary difference

// Tertiary

options.strength = Some(Strength::Tertiary);
options.case_level = Some(CaseLevel::Off);
let tertiary =
  Collator::try_new(&Default::default(),
                    options).unwrap();

assert_eq!(tertiary.compare("ⓓⓔⓐⓛ", "DEAL"), Ordering::Less);
assert_eq!(tertiary.compare("dejavu", "dejAvu"), Ordering::Less);
assert_eq!(tertiary.compare("dejavu", "déjavu"), Ordering::Less);
```

### Case First

Whether to swap the ordering of uppercase and lowercase.

### Backward second level

Compare the second level in backward order. The default is `false` (off), except for Canadian
French.

### Numeric

When set to `true` (on), any sequence of decimal
digits is sorted at a primary level according to the
numeric value.

```rust
use core::cmp::Ordering;
use icu::collator::*;

// Numerical sorting off

let mut options_num_off = CollatorOptions::new();
options_num_off.numeric = Some(Numeric::Off);
let collator_num_off: Collator =
    Collator::try_new(&Default::default(), options_num_off).unwrap();
assert_eq!(collator_num_off.compare("a10b", "a2b"), Ordering::Less);

// Numerical sorting on

let mut options_num_on = CollatorOptions::new();
options_num_on.numeric = Some(Numeric::On);
let collator_num_on: Collator =
    Collator::try_new(&Default::default(), options_num_on).unwrap();
assert_eq!(collator_num_on.compare("a10b", "a2b"), Ordering::Greater);
```

<!-- cargo-rdme end -->

## More Information

For more information on development, authorship, contributing etc. please visit [`ICU4X home page`](https://github.com/unicode-org/icu4x).
