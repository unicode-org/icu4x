# WASM vs Transpilation Approach

We've considered two approaches to a problem of write logic once, deploy it on multiple platforms/languages with minimal effort.

### Options to start from

We should think about various approaches outside of plain Rust->Wasm and Clojure->All, i.e:

* Rust->Others
* Clojure->Rust->Wasm
* C++->Wasm
* C++->Others
* Comparison of basic approaches
* Comparison of pros/cons of each proposed approach.

***
*NOTE*: Current decision is to develop Rust based library to fulfill immediate Mozilla and Fuchsia needs. Next steps involve Wasm compilation, and potential transpilation for parts of library that requires high performance.
***

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
|Process to add new languages| Defer to the target language's ecosystem to adopt WASM bindings, then once that is ready, add the glue code to the ICU4X project. The contributor needs to be fluent in the target language and familiar with its WASM bindings.| Write a transpiler from ICU4X Rust source to the target language, then add the glue code; both pieces go into the ICU4X project.  The contributor needs to be fluent in the target language and also conversant in Rust.|
|Maintenance| We maintain only core i18n logic. Rust/Wasm toolchain is maintained by their community.| We maintain both core i18n logic and transpiler logic (AST to native code).|

