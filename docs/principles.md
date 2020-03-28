# OmnICU Design Principles & Decisions

## Guiding principles

### Immutable objects

Discussion about builders, constructors. But no object is to be mutated after creation.

### No I/O in the core library

Leave I/O to the wrapper or clients. Pass data as reference to the methods.
Clients can mmap, async fetch, package data with the library...

### No internal threading

Both Clojure and Wasm support multithreading but we don’t have a need for it in the i18n realm.

To simplify our library, and make sure we don’t have cross platform/language compatibility issues, one should avoid using threads in the core library.

### No global mutable data

ICU has an internal cache that optimizes construction of some of the objects. We plan to leave optimization to the user, i.e. reusable objects should be kept around for later use, not discarded and recreated again.

### Stable data across library versions

One of the big problems for existing ICU users is that ICU data cannot be shared among different versions of code, forcing clients to carry hefty duplicates with small deltas.

Another approach is to deploy deltas on data for each installed version of the library. This approach is hard, given data can be cut differently, e.g. with or without currency data.

### Use Heuristics and ML

ML may be required for inflection support, or to improve line breaking and date/number parsing. Service approach is also in play here.

Main takeaway is to not prohibit ML use in the library.

### Modular Code and Data with static analysis

Both the code and the data should be written so that you only bring what you need.  Code and data should be modular not only on a "class" level, but also within a class, such that you don't carry code and data for a feature of a class that you aren't using.

Code and data slicing should be able to be determined using static code analysis.  We should be able to look at the functions being called, and from that, build exactly the code and data bundle that the app needs.

### Options to start from

We should think about various approaches outside of plain Rust->Wasm and Clojure->All, i.e:

* Rust->Others
* Clojure->Rust->Wasm
* C++->Wasm
* C++->Others
* Comparison of basic approaches
* Comparison of pros/cons of each proposed approach.

NOTE: Current decision is to develop Rust based library to fulfill immediate Mozilla and Fuchsia needs. Next steps involve Wasm compilation, and potential transpilation for parts of library that requires high preformance.

### Comparison of basic approaches

Comparison of pros/cons of each proposed approach.

||Rust + WASM|Transpiler to native code|
|:--|:--|:--|
|Integration & build| WASM targets are pre-generated and client needs to add them to the build environment. We may have a combinatorial explosion with increasing API surface.| Transpiling produces native code which is easy to integrate into existing client build environment.|
|Debugging| Points of failure: Rust logic bug, Wasm compiler bug, Wasm runtime bug, glue code bug. Need to research Wasm debugging tools.| Points of failure: source language logic bug, transpiler bug, glue code bug. Positive: Use native tools to debug.|
|Code sharing/dynamic libraries| Code across WASM targets is not shared, e.g. vector impl will be included two times for two targets that use it. Work in progress on dynamic libraries.| Behaves the same as target language, so code sharing is not an issue.|
|Tree shaking| No tree shaking after Rust generates WASM.| Native compiler would do dead code elimination and tree shaking (Dart, Go and JS compiler as good examples).|
|Speed| Close to native, but depends on runtime. Go runtimes have 70x speed difference.| JVM (Java and derivatives), native (Rust, C++) or JIT (JS)|
|Support (platform/language)| Wasm support is expanding. Runtime quality is questionable, except for JS, C++, Rust.| Platform specific tweaks may be necessary (C++ version, compiler support for language constructs).|
|Process to add new languages| Defer to the target language's ecosystem to adopt WASM bindings, then once that is ready, add the glue code to the OmnICU project. The contributor needs to be fluent in the target language and familiar with its WASM bindings.| Write a transpiler from OmnICU Rust source to the target language, then add the glue code; both pieces go into the OmnICU project.  The contributor needs to be fluent in the target language and also conversant in Rust.|
|Maintenance| We maintain only core i18n logic. Rust/Wasm toolchain is maintained by their community.| We maintain both core i18n logic and transpiler logic (AST to native code).|
|||