ICU4X Charter
==============

The ICU4X project designs and develops a set of modular internationalization components suitable for use in client-side and resource-constrained environments.

ICU4X builds on the design and API decisions of [ICU4C]/[ICU4J] and [ECMA-402].

## Design

ICU4X is being built from the start with several key design constraints:

1. Small, modular, fast, low-memory code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by internationalization experts to encourage best practices.

Above all, ICU4X code will produce correct results for all languages and locales. No language or locale should be at a structural disadvantage.

## Scope

The scope of ICU4X is the set of functionality covered by [ECMA-402], with extensions to cover the needs of clients including Gecko and Fuchsia that are consistent with the spirit of [ECMA-402].

APIs included in ICU4X will:

* Be useful to a broad audience of client-side applications
* Depend on Unicode or CLDR data or algorithms
* Benefit from being versioned alongside Unicode and CLDR data

The feature coverage of [ICU4C] and [ICU4J] goes well beyond what is necessary for the target clients; ICU4X will have a more narrow focus. For such APIs and functionality that will not end up being included, ICU4X aims to provide the building blocks to enable third-party libraries to be developed which depend on and interoperate with ICU4X, with minimum data and logic duplication. For example, ICU4X will not include Spoof Checker (UTS 39) or IDNA (UTS 46), but third-party libraries could build on top of ICU4X's Unicode Properties APIs to implement those features.

In some rare cases, ICU4X may diverge from both [ECMA-402], and [ICU4C]/[ICU4J]. Such diversions should be the result of improved API design and should allow users to cover the same needs with the upgraded APIs.

## Target platforms

The list of target platforms is expected to evolve over time, and will be primarily used when making decisions on features and API tradeoffs between low-level ICU4X-like APIs and high-level [ECMA-402] APIs.

Current list of target platforms:
* Web Platform (V8, SpiderMonkey, JSC)
* Software platforms (Fuchsia, Gecko)
* Mobile OSes (iOS, Android)
* Low-power OSes with [alloc] (WearOS, WatchOS)
* Client-side toolkits (Flutter)

ICU4X is also aiming to provide bindings or compilation targets for a range of programming languages. This list is also dynamic and used when evaluating design tradeoffs:

* Rust
* JavaScript
* Objective-C
* Java
* Dart
* C++

Referential platforms and languages may change over time as the industry developers and should represent the current needs of the industry.

A viable subset of ICU4X will be targeting the [no_std] support and in the future we may explore the [no_std]+[alloc] compatibility for viable components ([#77](https://github.com/unicode-org/icu4x/issues/77)).

## Frequently Asked Questions

### What will be the organizational structure of ICU4X?

The ICU4X sub-committee (ICU4X-SC), a sub-committee of the ICU technical committee (ICU-TC), will be composed of members of the Unicode Consortium.  The ICU4X-SC will make architectural decisions consistent with this charter, and guide the development and maintenance of the project.

ICU4X will have an independent code base from ICU, and will operate independently of the ICU-TC. It will need no support from the the core staff of the Unicode Consortium except an occasional announcement.

### Is ICU4X going to replace ICU?

ICU4X is a new library to fill the growing need for on-device internationalization across a variety of client-side platforms, including IoT, mobile, and web environments.  We hope ICU4X will eventually replace client-side solutions such as Closure i18n (goog.i18n) and Dart Intl.  [ICU4C] and [ICU4J] will continue to be the gold standard for internationalization on servers and higher-resource environments.

### Why make a new project instead of improving ICU?

Because client-side needs are fundamentally different from server-side needs.

ICU has a long history of contributors adding [cruft](http://site.icu-project.org/design/cpp#TOC-Cruft-Complication) to serve their specific use cases.  Over that time, ICU has been increasingly optimized for high performance on servers; code size and constrained memory usage have not been at the forefront of design decisions.

Improving ICU to be the go-to solution to solve all client-side use cases would amount to a major overhaul and rewrite of the software, including a complete overhaul of the data loading mechanism, a re-write of classes with large code size, disentanglement of Java class dependencies, and a refactor of singleton caches to reduce memory usage.  In addition to these each being multi-quarter efforts, they would need to be done in a way that is backwards-compatible and does not hurt performance on the server side.  Such an effort also would not solve the current need from clients like Fuchsia and Mozilla for a memory-safe Rust ICU.  At the end of the day, given the interconnectedness of ICU primitives, even with compile-time dead code elimination, we will never be able to get code size down to as low as if we wrote new code with that as a primary design constraint.

A new project allows us to build in the client-side needs from day 1, including small code size and pluggable data dependencies, in a way that works well on all of the target platforms, without introducing further cruft to ICU in the process.

### How will you actually implement ICU4X?

Internally, the first draft of ICU4X will be built in a [no_std] Rust Environment, with plans to use either FFI or [WebAssembly](https://webassembly.org/) for porting the code to the other target platforms; however, we are leaving the door open for other options such as transpilation that can produce code directly.  For example, we expect Rust clang/llvm tools to stabilize in the future, which could be the basis of a potential transpilation effort.  We are also looking at a Lisp-like source that can generate high-level code.

### Why not call it "icu4rust"?

The implementation language is intended to be an abstract concept: an internal detail which could change over time.  The focus of ICU4X is on the clients: the web platform, Android, iOS, and the other platforms listed earlier.  Rust, in the context Fuchsia and Gecko, is just one of many clients.

### Why consider Lisp for a transpiler?

A transpiler can be written in any language, but Lisp has a history of being used in high leverage applications like this, and its S-expression syntax is useful in compilers, etc. A modern dialect like Clojure has growth, tooling, and large library access. A one-to-many transpiler, as needed here, has more inherent complexity than just a one-to-one transpiler, so Lisp would maximize chances for success.  [See here](https://elangocheran.com/2020/03/18/why-clojure-lisp-is-good-for-writing-transpilers/) for more info.

### Won't this increase the maintenance burden?

Internationalization engineers currently need to maintain several half-baked client-side internationalization solutions including Closure i18n and Dart Intl.  We hope ICU4X will be able to eventually replace those libraries, and it will also allow us to fulfill the needs of new clients that we are currently unable to support.

### Will ICU4C, ICU4J, and ICU4X share any code?

Since C++ and Java are both target output languages of ICU4X, it is possible that certain core algorithms can be implemented in ICU4X first and then shipped under the hood with [ICU4C]/[ICU4J].  In [ICU4C], Rust could be statically built into the [ICU4C] shared object files, such that it is transparent to [ICU4C] clients.  This is a theoretical possibility that will need to be evaluated by the ICU-TC at a later date.

### Why not put it in the ICU repository governed directly by ICU-TC?

ICU4X will have some overlap of personnel with ICU, but the processes, builds, and release cycle will be run separately from ICU.  The ICU repository is closely tied to the ICU release processes, with each pull request running the [ICU4C] and [ICU4J] test suites, linked to Jira issues.

[ICU4C]: https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/
[ICU4J]: https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/
[ECMA-402]: https://www.ecma-international.org/publications/standards/Ecma-402.htm
[no_std]: https://rust-embedded.github.io/book/intro/no-std.html
[alloc]: https://doc.rust-lang.org/alloc/

