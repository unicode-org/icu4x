ICU4X Charter
==============

ICU4X is a new project whose objective is to solve the needs of clients who wish to provide client-side i18n for their products in resource-constrained environments.

ICU4X will be built from the start with several key design constraints:

1. Small and modular code.
2. Pluggable locale data.
3. Availability and ease of use in multiple programming languages.
4. Written by i18n experts to encourage best practices.

ICU4X will provide an [ECMA-402](https://www.ecma-international.org/publications/standards/Ecma-402.htm)-compatible API surface in the target client-side platforms, including the web platform, iOS, Android, WearOS, WatchOS, Flutter, and Fuchsia, supported in programming languages including Rust, JavaScript, Objective-C, Java, Dart, and C++.

## Frequently Asked Questions

### What will be the organizational structure of ICU4X?

The ICU4X sub-committee (ICU4X-SC), a sub-committee of the ICU technical committee (ICU-TC), will be composed of members of the Unicode Consortium.  The ICU4X-SC will make architectural decisions consistent with this charter, and guide the development and maintenance of the project.

ICU4X will have an independent code base from ICU, and will operate independently of the ICU-TC. It will need no support from the the core staff of the Unicode Consortium except an occasional announcement.

### Is ICU4X going to replace ICU?

No!

ICU4X is a new library to fill the growing need for on-device i18n across a variety of client-side platforms, including IoT, mobile, and web environments.  We hope ICU4X will eventually replace client-side solutions such as Closure i18n (goog.i18n) and Dart Intl.  ICU4C and ICU4J will continue to be the gold standard for internationalization on servers and higher-resource environments.

### Why make a new project instead of improving ICU?

Because client-side needs are fundamentally different from server-side needs.

ICU has a long history of contributors adding [cruft](http://site.icu-project.org/design/cpp#TOC-Cruft-Complication) to serve their specific use cases.  Over that time, ICU has been increasingly optimized for high performance on servers; code size and constrained memory usage have not been at the forefront of design decisions.

Improving ICU to be the go-to solution to solve all client-side use cases would amount to a major overhaul and rewrite of the software, including a complete overhaul of the data loading mechanism, a re-write of classes with large code size, disentanglement of Java class dependencies, and a refactor of singleton caches to reduce memory usage.  In addition to these each being multi-quarter efforts, they would need to be done in a way that is backwards-compatible and does not hurt performance on the server side.  Such an effort also would not solve the current need from clients like Fuchsia and Mozilla for a memory-safe Rust ICU.  At the end of the day, given the interconnectedness of ICU primitives, even with compile-time dead code elimination, we will never be able to get code size down to as low as if we wrote new code with that as a primary design constraint.

A new project allows us to build in the client-side needs from day 1, including small code size and pluggable data dependencies, in a way that works well on all of the target platforms, without introducing further cruft to ICU in the process.

### How will you actually implement ICU4X?

Internally, the first draft of ICU4X will be built in a [no_std Rust Environment](https://rust-embedded.github.io/book/intro/no-std.html), with plans to use either FFI or [WebAssembly](https://webassembly.org/) for porting the code to the other target platforms; however, we are leaving the door open for other options such as transpilation that can produce code directly.  For example, we expect Rust clang/llvm tools to stabilize in the future, which could be the basis of a potential transpilation effort.  We are also looking at a Lisp-like source that can generate high-level code.

### Why not call it "icu4rust"?

The implementation language is intended to be an abstract concept: an internal detail which could change over time.  The focus of ICU4X is on the clients: the web platform, Android, iOS, and the other platforms listed earlier.  Rust, in the context Fuchsia and Gecko, is just one of many clients.

### Why consider Lisp for a transpiler?

A transpiler can be written in any language, but Lisp has a history of being used in high leverage applications like this, and its S-expression syntax is useful in compilers, etc. A modern dialect like Clojure has growth, tooling, and large library access. A one-to-many transpiler, as needed here, has more inherent complexity than just a one-to-one transpiler, so Lisp would maximize chances for success.  [See here](https://elangocheran.com/2020/03/18/why-clojure-lisp-is-good-for-writing-transpilers/) for more info.

### Won't this increase the maintenance burden?

I18n engineers currently need to maintain several half-baked client-side i18n solutions including Closure i18n and Dart Intl.  We hope ICU4X will be able to eventually replace those libraries, and it will also allow us to fulfill the needs of new clients that we are currently unable to support.

### Will ICU4C, ICU4J, and ICU4X share any code?

Since C++ and Java are both target output languages of ICU4X, it is possible that certain core algorithms can be implemented in ICU4X first and then shipped under the hood with ICU4C/ICU4J.  In ICU4C, Rust could be statically built into the ICU4C shared object files, such that it is transparent to ICU4C clients.  This is a theoretical possibility that will need to be evaluated by the ICU-TC at a later date.

### Why not put it in the ICU repository governed directly by ICU-TC?

ICU4X will have some overlap of personnel with ICU, but the processes, builds, and release cycle will be run separately from ICU.  The ICU repository is closely tied to the ICU release processes, with each pull request running the ICU4C and ICU4J test suites, linked to Jira issues.

### What if clients need a feature that is not in ECMA-402?

Clients of ICU4X may need features beyond those recommended by ECMA-402.  The subcommittee is not ruling out the option of adding additional features in the same style as ECMA-402 to cover additional client needs.  The details for how to determine what features belong in ICU4X that aren't already in ECMA-402 will be discussed at a future time.
