Data Safety in ICU4X
====================

At its core, ICU4X is a set of algorithms that map input data to human-ready outputs. Since a core value proposition of ICU4X is the ability to dynamically load data, we can't always assume that our data are valid and pristine. This doc covers how we think about this problem and handle it in ICU4X.

## Resolution

1. _Whereas_ it is rare to be 100% confident about the safety of your data\*; and
1. _Whereas_ validating data invariants can be expensive; and
1. _Whereas_ moving validation into the algorithm (post-deserialization) often reduces the overall overhead of validation; and
1. _Whereas_ it is useful to _be able to_ learn whether code traverses unexpected code paths; and
1. _Whereas_ end users of ICU4X algorithms are not in a position to reason about invalid data in otherwise-infallible terminal functions; and
1. _Whereas_ algorithms that panic on malformed data increase the vulnerability space of an application\*\*; and
1. _Whereas_ "garbage in, garbage out" (GIGO) does not increase the space that malicious actors could leverage\*\*; be it
1. _Resolved_ that code should never panic at runtime based on invalid data; and be it
1. _Resolved_ that data structs should reduce the number of internal invariants; and be it
1. _Resolved_ that code paths only reachable by invalid data should use GIGO with debug assertions.

\* *As a thought experiment, if you were 100% confident, you could use `get_unchecked` and other unsafe operations. If you are not confident enough to use unsafe code, then you are not 100% confident.*

\*\* *An attacker could construct data to produce a result they desire, regardless of whether unexpected operations end in GIGO. However, panicky algorithms could enable attackers to perform denial-of-service attacks.*

Additional points:

1. If performing binary search on a vector from data, it is not necessary to validate that the vector is sorted. The binary search will fail to find elements in an unsorted vector, but it won't panic, so this is consistent with GIGO.
2. The best data structs are those that don't need to be validated. See the below example on how to write data structs without internal invariants.

## Example

Consider the struct

```rust
#[derive(Deserialize)]
pub struct WeekdayNamesBad {
    // Invariant 1: the first element in the vector is the default value
    // Invariant 2: there are 7 elements in the vector
    weekday_names: Vec<String>,
}

impl WeekdayNamesBad {
    pub fn get_first_weekday_name(&self) -> &str {
        &self.weekday_names[0]
    }
    pub fn get_weekday_name_at_index(&self, idx: usize) -> Option<&str> {
        if idx >= 7 {
            return None;
        }
        Some(&self.weekday_names[idx])
    }
}
```

This struct has 2 invariants, and each of the functions could panic if those invariants are not upheld.

Below are 3 approaches to resolve this issue that are consistent with the ICU4X style.

### Solution 1: Garbage In, Garbage Out

Change the functions to return default fallback values if the data is not in the expected form.

```rust
#[derive(Deserialize)]
pub struct WeekdayNamesGIGO {
    // WEAK Invariant 1: the first element in the vector is the default value
    // WEAK Invariant 2: there are 7 elements in the vector
    weekday_names: Vec<String>,
}

impl WeekdayNamesGIGO {
    pub fn get_first_weekday_name(&self) -> &str {
        match self.weekday_names.get(0) {
            Some(v) => v,
            None => {
                debug_assert!(false, "weekday_names is empty");
                ""
            }
        }
    }
    pub fn get_weekday_name_at_index(&self, idx: usize) -> Option<&str> {
        self.weekday_names.get(idx).as_ref()
    }
}
```

### Solution 2: Remove Internal Invariants

Change the struct so that it doesn't have internal invariants.

```rust
#[derive(Deserialize)]
pub struct WeekdayNamesNoInvariants {
    pub first_weekday_name: String,
    pub remaining_weekdays: Vec<String>,
}

impl WeekdayNamesNoInvariants {
    pub fn get_first_weekday_name(&self) -> &str {
        &self.first_weekday_name
    }
    pub fn get_weekday_name_at_index(&self, idx: usize) -> Option<&str> {
        if idx == 0 {
            Some(&self.first_weekday_name)
        } else {
            self.remaining_weekdays.get(idx - 1).as_ref()
        }
    }
}
```

This struct has no internal invariants. The deserialization step automatically enforces that the default value is present.

### Solution 3: Use Helper Struct

With this solution, check that there are the expected number of items at deserialization time. Leverage Serde's [try_from](https://serde.rs/container-attrs.html#try_from) attribute to perform the validation.

```rust
#[derive(Deserialize)]
struct WeekdayNamesInner {
    pub weekday_names: Vec<String>,
}

#[serde(try_from = WeekdayNamesInner)]
pub struct WeekdayNamesWrap {
    // Invariant: the vector is non-empty.
    weekday_names: Vec<String>,
}

impl TryFrom<WeekdayNamesInner> for WeekdayNamesWrap {
    type Error = DataError;
    pub fn try_from(other: WeekdayNamesInner) -> Result<Self, Self::Error> {
        if other.is_empty() {
            Err(DataError::custom("weekday_names must not be empty"))
        } else {
            Ok(Self { weekday_names: other.weekday_names })
        }
    }
}

impl WeekdayNamesWrap {
    pub fn get_first_weekday_name(&self) -> &str {
        #[allow(clippy::indexing_slicing)] // validated invariant
        &self.weekday_names[0]
    }
    pub fn get_weekday_name_at_index(&self, idx: usize) -> Option<&str> {
        self.weekday_names.get(idx).as_ref()
    }
}
```
