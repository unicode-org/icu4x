# ICU4X Design Principles & Decisions

## Guiding principles

These principles are not cast in stone, but are strong guidelines for developers.

### Immutable objects

No object is to be mutated after creation.

### No I/O in the core library

Leave I/O to the wrapper or clients. Pass data as reference to the methods.

***
*NOTE*: Clients can mmap, async fetch, package data with the library...
***

### No internal threading

Both Rust and Wasm support multithreading but we don’t have a need for it in the i18n realm.

To simplify our library, and make sure we don’t have cross platform/language compatibility issues, one should avoid using threads in the core library.

***
*NOTE*: In rare cases where threading is needed in native Rust implementation, multiple code paths can be created/opted in at build time.
***

### No global mutable data

ICU has an internal cache that optimizes construction of some of the objects. We plan to leave optimization to the user, i.e. reusable objects should be kept around for later use, not discarded and recreated again.

***
*NOTE*: Memoization is acceptable, as it's not really a global cache, but an object with internal state.  An example implementation used in [fluent-rs](https://github.com/projectfluent/fluent-rs/tree/master/intl-memoizer).
***

### Stable data across library versions

One of the big problems for existing ICU users is that ICU data cannot be shared among different versions of code, forcing clients to carry hefty duplicates with small deltas.

Another approach is to deploy deltas on data for each installed version of the library. This approach is hard, given data can be cut differently, e.g. with or without currency data.

### Use Heuristics and ML

ML may be required for inflection support, or to improve line breaking and date/number parsing. Service approach is also in play here.

Main takeaway is to not prohibit ML use in the library.

### Modular Code and Data with static analysis

Both the code and the data should be written so that you only bring what you need.  Code and data should be modular not only on a "class" level, but also within a class, such that you don't carry code and data for a feature of a class that you aren't using.

Code and data slicing should be able to be determined using static code analysis.  We should be able to look at the functions being called, and from that, build exactly the code and data bundle that the app needs.

