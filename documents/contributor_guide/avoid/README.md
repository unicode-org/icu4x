# Avoid rules

Short rules for ICU4X code. Each rule file has ❌/✅ examples, the reason,
the exceptions, and how the rule is enforced. IDs and file names never
change. A retired rule keeps its file and points to its replacement.

| ID | Avoid | Enforced by | Source | Status |
|---|---|---|---|---|
| [AV001](AV001_no_panics_in_library_code.md) | `unwrap`, `expect`, indexing, or `panic!` in library code | Clippy | style_guide.md: Don't Panic, Lints › Panics | Proposed |
| [AV002](AV002_no_needless_allocation.md) | Allocating when you can borrow: `String` results, `to_string`, taking `&T` to clone it; return `Cow` or a `Writeable` | Review | style_guide.md: Avoid implicit allocations, Avoid `to_string` | Proposed |
| [AV003](AV003_no_std_in_library_crates.md) | `std` in library crates; use `core`/`alloc`, and add a `std` feature only for code that needs `std` | Compiler (`no_std`), `ci-job-nostd` | principles.md: No std in core library; boilerplate.md; graduation.md | Proposed |
| [AV004](AV004_zero_copy_data_structs.md) | `String`/`Vec`/maps in data structs, or `Cow` without `#[serde(borrow)]` | CI zero-copy check | style_guide.md: Zero-copy in DataProvider structs; graduation.md | Proposed |
| [AV005](AV005_stable_serialized_layout.md) | Changing the serialized layout of a released data struct | Review (`fingerprints.csv` diff) | data_versioning.md; #8250 | Proposed |

AV006 to AV015 are reserved for the other candidate rules in §5 of the
design doc ([#8557](https://github.com/unicode-org/icu4x/pull/8557)).
