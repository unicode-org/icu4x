Data Safety in ICU4X
====================

At its core, ICU4X is a set of algorithms that map input data to human-ready outputs. Since a core value proposition of ICU4X is the ability to dynamically load data, we can't always assume that our data are valid and pristine. This doc covers how we think about this problem and handle it in ICU4X.

## Problem and Policy Statement

Given the following evidence…

1. It is rare to be 100% confident about the safety of your data\*
1. Validating data invariants can be expensive
1. Moving validation into the algorithm (post-deserialization) often reduces the overall overhead of validation
1. It is useful to _be able to_ learn whether code traverses unexpected code paths
1. End users of ICU4X algorithms are not in a position to reason about invalid data in otherwise-infallible terminal functions
1. Algorithms that panic on malformed data increase the vulnerability space of an application\*\*
1. "Garbage in, garbage out" (GIGO) does not increase the space that malicious actors could leverage\*\*

…the ICU4X project has adopted the following policy:

1. Code should never panic at runtime based on invalid data; and be it
1. Data structs should reduce the number of internal invariants; and be it
1. Code paths only reachable by invalid data should use GIGO with debug assertions.

\* *As a thought experiment, if you were 100% confident, you could use `get_unchecked` and other unsafe operations. If you are not confident enough to use unsafe code, then you are not 100% confident.*

\*\* *An attacker could construct data to produce a result they desire, regardless of whether unexpected operations end in GIGO. However, panicky algorithms could enable attackers to perform denial-of-service attacks.*

Additional points:

1. If performing binary search on a vector from data, it is not necessary to validate that the vector is sorted. The binary search will fail to find elements in an unsorted vector, but it won't panic, so this is consistent with GIGO.
2. The best data structs are those that don't need to be validated. See the below example on how to write data structs without internal invariants.

## Example

Consider the struct

```rust
#[derive(serde::Deserialize)]
pub struct MonthNamesBad {
    // Invariant 1: the first element in the vector is the default value
    // Invariant 2: the vector contains 1 element per month in the current calendar
    month_names: Vec<String>,
    month_count: usize,
}

impl MonthNamesBad {
    pub fn get_first_month_name(&self) -> &str {
        &self.month_names[0]
    }
    /// Returns None only if `idx` is greater than the month count.
    pub fn get_month_name_at_index(&self, idx: usize) -> Option<&str> {
        if idx >= self.month_count {
            return None;
        }
        Some(&self.month_names[idx])
    }
    pub fn get_month_count(&self) -> usize {
        self.month_count
    }
}
```

This struct has 2 invariants, and each of the functions could panic if those invariants are not upheld.

Below are 3 approaches to resolve this issue that are consistent with the ICU4X style.

*Editor's note: the examples in this document are tested along with the tutorials.*

### Solution 1: Garbage In, Garbage Out

Change the functions to return default fallback values if the data is not in the expected form. This code also consolidates `month_count` and `month_names` into the same field.

```rust
#[derive(serde::Deserialize)]
pub struct MonthNamesGIGO {
    // WEAK Invariant 1: the first element in the vector is the default value
    // WEAK Invariant 2: the vector length is the count of months in the current calendar
    month_names: Vec<String>,
}

impl MonthNamesGIGO {
    pub fn get_first_month_name(&self) -> &str {
        match self.month_names.get(0) {
            Some(v) => v,
            None => {
                debug_assert!(false, "month_names is empty");
                ""
            }
        }
    }
    pub fn get_month_name_at_index(&self, idx: usize) -> Option<&str> {
        self.month_names.get(idx).map(String::as_str)
    }
    pub fn get_month_count(&self) -> usize {
        self.month_names.len()
    }
}
```

### Solution 2: Remove Internal Invariants

Change the struct so that it doesn't have internal invariants.

```rust
#[derive(serde::Deserialize)]
pub struct MonthNamesNoInvariants {
    pub first_month_name: String,
    pub remaining_months: Vec<String>,
}

impl MonthNamesNoInvariants {
    pub fn get_first_month_name(&self) -> &str {
        &self.first_month_name
    }
    pub fn get_month_name_at_index(&self, idx: usize) -> Option<&str> {
        if idx == 0 {
            Some(&self.first_month_name)
        } else {
            self.remaining_months.get(idx - 1).map(String::as_str)
        }
    }
    pub fn get_month_count(&self) -> usize {
        self.remaining_months.len() + 1
    }
}
```

This struct has no internal invariants. The deserialization step automatically enforces that the default value is present.

### Solution 3: Use Helper Struct

With this solution, check that there are the expected number of items at deserialization time. Leverage Serde's [try_from](https://serde.rs/container-attrs.html#try_from) attribute to perform the validation.

```rust
use icu_provider::DataError;

#[derive(serde::Deserialize)]
struct MonthNamesInner {
    pub month_names: Vec<String>,
}

#[derive(serde::Deserialize)]
#[serde(try_from = "MonthNamesInner")]
pub struct MonthNamesCheckedInvariants {
    // Invariant: the vector is non-empty.
    month_names: Vec<String>,
}

impl TryFrom<MonthNamesInner> for MonthNamesCheckedInvariants {
    type Error = DataError;
    fn try_from(other: MonthNamesInner) -> Result<Self, Self::Error> {
        if other.month_names.is_empty() {
            Err(DataError::custom("month_names must not be empty"))
        } else {
            Ok(Self { month_names: other.month_names })
        }
    }
}

impl MonthNamesCheckedInvariants {
    pub fn get_first_month_name(&self) -> &str {
        #[allow(clippy::indexing_slicing)] // validated invariant
        &self.month_names[0]
    }
    pub fn get_month_name_at_index(&self, idx: usize) -> Option<&str> {
        self.month_names.get(idx).map(String::as_str)
    }
    pub fn get_month_count(&self) -> usize {
        self.month_names.len()
    }
}
```
