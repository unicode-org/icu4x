Data Safety in ICU4X
====================

At its core, ICU4X is a set of algorithms that map input data to human-ready outputs. Since a core value proposition of ICU4X is the ability to dynamically load data, we can't always assume that our data are valid and pristine. This doc covers how we think about this problem and handle it in ICU4X.

## Resolution

1. _Whereas_ it is rare to be 100% confident about the safety of your data\*; and
1. _Whereas_ validating data invariants can be expensive; and
1. _Whereas_ moving validation into the algorithm (post-deserialization) often reduces the overall overhead of validation; and
1. _Whereas_ it is useful to _be able to_ learn whether code traverses unexpected code paths; and
1. _Whereas_ end users of ICU4X algorithms are not in a position to reason about invalid data in otherwise-infallible terminal functions; and
1. _Whereas_ if an algorithm processing data uses GIGO, it does not increase the space that malicious actors could leverage**; and
1. _Whereas_ algorithms that _panic_ on malformed data actually increase the vulnerability space of an application; be it
1. _Resolved_ that code should never panic at runtime based on invalid data; and be it
1. _Resolved_ that data structs should reduce the number of internal invariants; and be it
1. _Resolved_ that code paths only reachable by invalid data should use debug assertions.

\* *As a thought experiment, if you were 100% confident, you could use `get_unchecked` and other unsafe operations. If you are not confident enough to use unsafe code, then you are not 100% confident.*

\*\* *An attacker could construct data to produce a result they desire, regardless of whether unexpected operations end in GIGO. However, panicky algorithms could enable attackers to perform denial-of-service attacks.*

## What This Means for ICU4X Developers

When writing data structs, reduce the number of internal invariants.

For example, consider the struct

```rust
pub struct WithInvariant {
    all_values: Vec<String>,
    // Invariant: must be in range of the length of all_values
    default_index: usize,
}

impl WithInvariant {
    pub fn get_default() -> &str {
        &self.all_values[default_index]
    }
}
```

This struct panics if the invariant is not upheld. Better would be to write

```rust
pub struct WithInvariant {
    pub default_value: String,
    pub other_values: Vec<String>,
}
```
