# ICU4X Project Announcement

We are thrilled to introduce a new project in the Software Internationalization Domain: `ICU4X`.

`ICU4X` aims to provide high quality internationalization components with a focus on
* Modularity
* Flexible data management
* Performance, memory, safety and size
* Universal access from programming languages and ecosystems ([FFI](https://en.wikipedia.org/wiki/Foreign_function_interface))

`ICU4X` draws from the experience of projects such as [ICU4C](https://icu.unicode.org/), [ICU4J](https://icu.unicode.org/), [ECMA-402](https://www.ecma-international.org/publications/standards/Ecma-402.htm), [CLDR](https://cldr.unicode.org/) and [Unicode](https://home.unicode.org/) and is primarily developed in [Rust](https://www.rust-lang.org/).

## Design

When selecting components, and designing APIs for ICU4X the team is looking at the design decisions from ECMA-402 ([Javascript Internationalization API](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl)) as a golden standard of modern, flexible internationalization API.

The second source of inspiration is the ICU4C/ICU4J APIs, two projects that have grown to fulfill the needs of major software systems.

ICU4X aims to balance a subset of the ICU4C/ICU4J with the simplicity and flexibility of ECMA-402.

As a data source for internationalization, ICU4X relies on the CLDR project, ensuring high level of compatibility between current implementations of ECMA-402, ICU4C and ICU4J.

## Innovations

With ability to leverage the experience of the projects mentioned above, the team aims to bring a number of innovations to the industry.

### Modularity

ICU4X is highly modular allowing users to easily select and chunk pieces of the project and load only necessary components and data. This will allow ICU4X to be used in systems with memory and data size constraints such as client-side ecosystems, low-powered devices and so on.

### Flexible Data Management

[Data management](https://github.com/unicode-org/icu4x/blob/main/documents/design/data_pipeline.md) is a complex subcomponent of every internationalization ecosystem. Inflexible data management forces users to select which data and which locales are going to be available to users, which is a practice that goes against the idea of universal access to the system.

What’s more, such architecture forces trade-offs and compromises in which benefiting some users puts additional cost on other users, companies and degrades performance.

ICU4X aims to tackle these issues with a flexible data management system which will allow for architectures to be more adaptable as code and data can be adjusted to the needs of an ecosystem, loaded synchronously and asynchronously, and updated over time.

We hope this approach will allow more software to be accessible to the whole World without forcing tough compromises on the product and project decision makers.

### Performance, Memory, Safety and Size

ICU4C and ICU4J evolved many highly performant algorithms and data structures which serve the internationalization components.

Rust is a modern, highly performant and safe programming language with a robust ecosystem of libraries allowing for low overhead and high performance.

ICU4X aims to leverage both and in result enable internationalization features to be available in highly constrained environments such where CPU, memory and power are critically important.

With the early focus on [benchmarking](https://github.com/unicode-org/icu4x/wiki/Benchmarking), we hope to achieve substantially lower memory and CPU overhead compared to current internationalization solutions without compromising feature completeness and global reach of the APIs. Initial results from our microbenchmarks and more quasi-real-life apps [looks very promising](https://github.com/zbraniecki/intl-measurements/).

### Universal Access (FFI)

With a breadth of programming languages and environments we see today, it is not scalable to rewrite internationalization components to each programming environment separately. The high cost of maintaining internationalization APIs and data is hard to justify by the industry and stakeholders.

ICU4X aims to bring a single canonical codebase, and well maintained data management to a wide range of programming languages.

Rust provides strong bindings to [WebAssembly](https://webassembly.org/), and we plan to expose ICU4X to JavaScript, Dart, C, and others.

## Target

While it’s early to have exact scope settled, ICU4X aims to expose a superset of `ECMA-402` scope of APIs to a wide range of platforms such as:

* Web Platform (V8, SpiderMonkey, JSC)
* Software platforms (Fuchsia, Gecko)
* Mobile OSes (iOS, Android)
* Low-power OSes with alloc (WearOS, WatchOS)
* Client-side toolkits (Flutter)

with bindings to languages such as:

* Rust
* WebAssembly
* JavaScript
* Objective-C
* Java
* Dart
* C++

The exact scope may change, as the project matures and languages and ecosystems gain and lose relevance, but the target of having a complete, modular and performant internationalization solution for many ecosystems will remain at the core of ICU4X.

*Notice*: That means that ICU4X does not plan to duplicate or replace ICU4C or ICU4J, as they will remain more complete. Instead ICU4X aims to be capable of backing ECMA-402 scope in many environments.

## Status

ICU4X has just released its first unstable [version 0.1](https://github.com/unicode-org/icu4x/releases/tag/icu%400.1.0). This version brings the first iteration of [`Data Provider`](https://docs.rs/icu_provider/0.1.0/icu_provider/), and a small selection of components in their early stages such as [`Locale` and `Language Identifiers`](https://docs.rs/icu/0.1.0/icu/locid/index.html), [`Plural Rules`](https://docs.rs/icu/0.1.0/icu/plurals/index.html), [`Unicode Set`](https://docs.rs/icu/0.1.0/icu/uniset/index.html) and [`Date & Time`](https://docs.rs/icu/0.1.0/icu/datetime/index.html) formatting.

To try ICU4X today, include it in `Cargo.toml`:

```toml
[dependencies]
icu = "0.1"
icu_provider_fs = "0.1"
```

and then:

```rust
use icu::locid::langid;
use icu::datetime::{DateTimeFormat, date::MockDateTime, options::length};
use icu_provider_fs::FsDataProvider;

fn main() {
    let lid = langid!("pl");

    let date: MockDateTime = "2020-10-14T13:21:00".parse()
        .expect("Failed to parse a datetime.");

    let provider = FsDataProvider::try_new("/home/{USER}/projects/icu/icu4x-data")
        .expect("Failed to initialize Data Provider.");

    let options = length::Bag {
        time: Some(length::Time::Medium),
        date: Some(length::Date::Long),
        ..Default::default()
    }.into();

    let dtf = DateTimeFormat::try_new(lid, &provider, &options)
        .expect("Failed to initialize DateTimeFormat");

    let formatted_date = dtf.format(&date);

    println!("📅: {}", formatted_date);
}
```

You can learn how to write such example app with the help of the [Introduction Tutorial](https://github.com/unicode-org/icu4x/wiki/Introduction-to-ICU4X-for-Rust).

The [next release](https://github.com/unicode-org/icu4x/issues/239) will focus on strengthening the performance, memory and size characteristics and providing access to the APIs from other ecosystems via FFI.

## Summary

ICU4X is a major project with challenging goals and a wide range of features that, if successful, may be foundational for the coming decade of the Internationalization Industry and Unicode. Worldwide access to software is an ideal that the Unicode Project has been founded on, and ICU4X aims to advance that goal.

As a project in a highly-complex knowledge domain, the project’s ability to succeed depends on its ability to unify and attract the whole industry to work together. As an open source project, ICU4X hopes to attract all stakeholders and interested parties to aid them in this journey and consider contributing to the project.

We are looking for internationalization experts, linguists, programming language architects and software engineers to help us make ICU4X a reality.

If you or your organization would like to contribute, you can find more information in the [CONTRIBUTING.md](https://github.com/unicode-org/icu4x/blob/main/CONTRIBUTING.md) document in [our github repository](https://github.com/unicode-org/icu4x).
