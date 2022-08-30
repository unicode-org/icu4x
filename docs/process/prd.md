# ICU4X 1.0 Product Requirement Document (PRD)

[ICU4X is an effort](https://github.com/unicode-org/icu4x/blob/master/docs/process/charter.md) under [Unicode](https://unicode.org/) guidance to develop a set of [Unicode Components](http://site.icu-project.org/) with focus on modularity, composability, and FFI supporting the scope of [ECMA-402](https://tc39.es/ecma402/) and written in accordance with modern internationalization techniques and standards.

The effort, currently driven mostly by engineering resources from Google and Mozilla, has been in development since February 2020 and we’re now looking to establish the criteria for evaluation of its value proposition and business fit.

The following is the vision for a stable, production ready 1.0 release and milestones on the path toward it.

## Value Proposition

Primarily, ICU4X aims to offer a more modular approach to internationalization, allowing client-side and resource-constrained environments to fine-tune the payload they vendor-in to maximize the quality and breadth of the internationalization coverage and relax the constraints internationalization imposes on product release cycles.

Secondarily, ICU4X aims to provide a single [high quality](https://github.com/unicode-org/icu4x/blob/master/docs/process/benchmarking.md) Internationalization API implementation written in a modern language, with [low memory overhead and great runtime performance](https://github.com/zbraniecki/intl-measurements/), exposed to many target ecosystems via robust FFI.
This will reduce the maintenance cost of internationalization stacks and improve access to high quality internationalization in multi-stack environments.

Finally, ICU4X will offer a powerful and flexible data management system, enabling sophisticated software release models and long-lived software processes to maintain data selection and updates.

## Stakeholders

The initial stakeholders are Unicode, Google and Mozilla.

Unicode provides the knowledge, experience and guidance, as well as a non-corporate environment in which such an industry wide effort can be developed.
Mozilla and Google provide engineering resources driving the development and offering business needs, product environment, and high-fidelity opportunities to validate the project against popular mature software.

As the project matures, we hope to attract three more classes of stakeholders:

### Internationalization Community

Thanks to Unicode guidance, strong engineering resources, mature CLDR base, and leveraging the experience of ECMA-402, ICU4C and ICU4J, ICU4X has a chance to expose the latest, modern and well designed APIs based on the lessons-learned of the last 30 years of industry development.

ICU4X will enable high quality, rich and modern internationalization for environments and software deployment types which are unable to benefit from the currently existing industry options.

With those options at hand, ICU4X should become an attractive project for the widely understood Internationalization Community, which currently often maintains pieces of ICU-like APIs for their target needs, or a patchwork of wrappers and heavily fine-tuned deployments of ICU.

### Major Software Vendors

Besides Google and Mozilla, many other organizations aim to target their products and services at the whole world.

Currently, most of them consume ICU, or maintain their own equivalent of such for environments where monolithic C++ and Java are not providing the right payload or safety characteristics.

With the maturing of ICU4X we hope to attract those organizations to consider ICU4X and contribute to it.

### Rust Community

Rust programming language has evolved one of the most unique and [highly productive software communities in the world](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/). Rust has been voted the most loved programming language in an annual StackOverflow survey for [four years in a row](https://stackoverflow.blog/2020/01/20/what-is-rust-and-why-is-it-so-popular/).

Internationalization is a notoriously challenging domain of software, and the Rust community has proven itself to handle challenging problems very well. Currently the ecosystem uses Unicode, and ICU-like APIs in many crucial places, including rustc compiler itself and multiple high quality layout implementations such as [xi-editor](https://github.com/xi-editor/xi-editor).
With the rising focus on higher-level libraries such as [GUI toolkits](https://www.areweguiyet.com/), [game and web engine](https://arewegameyet.rs/) components, high-quality internationalization solutions are increasingly in demand.

By introducing ICU4X to the community, we have a chance to attract high-quality contributors to ICU4X, much like we were able to build a robust community around ECMA-402.

## Current Status

[ICU4X 0.1 has been released](https://home.unicode.org/announcing-icu4x-0-1/) on October 23rd 2020. The release contained a smallest viable subset of API surface and design model to establish a release.

ICU4X 0.1’s selection of components aimed to:

-   Validate low-level models around [data management](https://github.com/unicode-org/icu4x/blob/master/docs/design/data_pipeline.md)
-   Establish project culture using simple building blocks such as [Locale](https://docs.rs/icu_locid/0.1.0/icu_locid/) and [Plural Rules](https://docs.rs/icu_plurals/0.1.0/icu_plurals/)
-   Introduce a low-level foundation for string operations with [CodePointSet](https://docs.rs/icu_uniset/0.1.0/icu_uniset/)
-   Expose a single, high-level, highly requested API - [DateTimeFormat](https://docs.rs/icu_datetime/0.1.0/icu_datetime/)
-   Release a meta-package [ICU](https://docs.rs/icu/0.1.0/icu/)

With the upcoming 0.2 release, we aim to close the gap between our 0.1 features and requirements of ECMA-402 for `Intl.Locale`, `Intl.PluralRules`, and `Intl.DateTimeFormat`, as discussed below.

## Risk Analysis

Developing a high-quality solution for the internationalization industry's needs is a noble goal which requires a lot of time to design correctly. All three current stakeholders are well positioned to justify such effort.

This allows ICU4X to strive for project quality rather than short-term business needs, but software projects disconnected from business needs are at risk of developing the [“Ivory Tower”](https://medium.com/it-dead-inside/knocking-down-the-ivory-tower-72fd249a8db7) syndrome which exposes risk of disconnection from real world alignment.

We strive create a roadmap that balances industry-level excellence with business alignments that allow us to frequently evaluate our ability to deliver on the value proposition. We want to recognize if the effort is not yielding expected results as early as possible.

To achieve this, we have identified a number of milestones, aligned with project planning, at which we will be able to test the prototypes of ICU4X against real business needs. This lets us validate and learn from the results, providing an opportunity to adapt our roadmap to ensure that ICU4X meets the needs of real-world products.

## Business Alignments

We identified a number of opportunities to evaluate progress of ICU4X on the path to 1.0 release against production environments:

### Test262

With the upcoming 0.2 release, ICU4X will provide at least three components that can be validated against a comprehensive internationalization test harness - [test262](https://github.com/tc39/test262) - used by ECMA-402:

-   [Intl.Locale](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Locale)
-   [Intl.PluralRules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/PluralRules)
-   [Intl.DateTimeFormat](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat)

The former two should be able to pass the full test scope.

DateTime API is larger, and we hope to be able to pass enough of the test corpus to make our performance measurements meaningful for comparison with production quality date and time formatting solutions.

If successful, this test will allow us to validate the claim that ICU4X can be a backend target for ECMA-402 level Internationalization APIs, and reason about performance characteristics of that subset of ICU4X.

### FFI Component

If we were to successfully expose a simple component of ICU4X via Wasm or FFI to another programming environment, such as Dart, JS, Python or PHP, we would be able to validate the claim that ICU4X can provide “write-once-use-everywhere” solution to low maintenance, high quality internationalization solutions in multiple environments.

The proposed target for such a component is a `FixedDecimal`. The chosen API is feature complete and allows us to validate mutable and immutable scenarios.

Such a component would require less data, have a simple API surface and require less code to be vendored in, while providing a high quality output for a highly requested feature.

If that test were to be successful, we would validate an additional ICU4X proposition: the ability to design internationalization components in a modular fashion, serving fully featured formatters to those who need to support full ECMA-402 and ICU4C/ICU4J needs, as well as serving a more modular subset of features to those who only need core functionality.

### Mozilla I18n

Mozilla Firefox Engine - Gecko - currently uses Rust components for [Locale](https://crates.io/crates/unic-langid) and [Plural Rules](https://crates.io/crates/intl_pluralrules).

Those components have been donated to ICU4X and their stripped versions served as a bedrock for the 0.1 release. Since then the implementations in ICU4X have been maturing and soon will reach feature and performance parity with the original libraries.

This will allow us to replace the standalone components with their evolved ICU4X derivatives, which will serve as a product market validation bringing ICU4X to 100m+ users on Windows, Linux, MacOS and Android.

### Web Engine Segmentation

As a result of the upcoming [ECMA-402 Intl.Segmenter API](https://github.com/tc39/proposal-intl-segmenter), the Mozilla Platform Internationalization Team is facing a challenge that ICU4X is directly aiming to solve.

Mozilla currently maintains [its own segmentation engine](https://searchfox.org/mozilla-central/source/intl/lwbrk) for its layout needs, and pulling in ICU4C Segmenter would [bring a substantial payload](https://bugzilla.mozilla.org/show_bug.cgi?id=1423593) and result in code and data duplication.

Replacing lwbrk with ICU4C for layout needs would require a substantial effort to customize data management to handle Mozilla’s custom data.

ICU4X offers an environment in which we are developing a new Unicode UAX#14/UAX#29 compatible segmentation API which will use the foundational [CodePointSet API](https://docs.rs/icu_uniset/0.1.0/icu_uniset/), and fit the needs of both Layout and ECMA-402.

Google contributed an ML based segmenter model for Thai, Burmese, Khmer and Lao, that cut down data size by ~75% and increased precision.

### Unicode Regular Expression Needs

Irregexp is a [Google regular expression engine](https://blog.chromium.org/2009/02/irregexp-google-chromes-new-regexp.html), used in Google and Mozilla [JS engines](https://hacks.mozilla.org/2020/06/a-new-regexp-engine-in-spidermonkey/).

Due to the problem known as [catastrophic backtracking](https://v8.dev/blog/non-backtracking-regexp) work is underway to develop a non-backtracking based engine as part of Irregexp. Irregexp currently uses ICU4C, but Mozilla is interested in investing in ICU4X Unicode Properties API to provide a [better aligned and more performant API](https://docs.google.com/document/d/1pcJc8joXpjE3sHwjPW9H1XbrCk8hKSL0IG-rzpAwNF4/edit) for irregexp needs and plans to implement the binding for irregexp to ICU4X.

### Fuchsia

Fuchsia maintains [a thin wrapper around ICU4C](https://crates.io/crates/rust_icu) exposed to Rust and would like to replace that with ICU4X. In case of a successful test262 April test, we’ll be in a good position to offer Fuchsia the ability to test a replacement of the same subset of APIs backed by ICU4C to ICU4X.

We do need to make sure to avoid data duplication, from existing ICU4C library and newly added ICU4X dependency. It would be great if ICU4X could use data already present in Fuchsia.

## ICU4X 1.0 Scope

In order to balance the technical and business requirements of the project, the following selection of APIs and features is proposed for the 1.0 release:

### DataProvider

Data Provider is the most crucial part of the ICU4X value proposition. High quality, flexible data management is required to prove the technical characteristics of the product, as well as provide business features that make ICU4X attractive to customers.

DataProvider is necessary to meet performance requirements, validate FFI models, and enable production use of ICU4X.

Additionally, flexible data provider gives users greater control over data / payload size and resource loading models.

For production readiness, we’ll need to validate the DataProvider management model using synchronous and asynchronous I/O, and prove the concept of chained data management.

### Locale

Locale is a fundamental data type in internationalization and an ECMA-402 component.

It validates ICU4X ability to design clean, ergonomic APIs and maintain a lean and modular architecture allowing customers to use just this component with or without data, if needed.

Locale is also important for our performance claims, since large software such as Firefox handle [hundreds of locale parsings and matchings during startup](https://docs.google.com/spreadsheets/d/1eWjslErJmzwGnWSX5eNXctjwa_KlK9QkyyM3NqPHgm8/edit#gid=1383258031) and a high quality of that component carries onto many hot-path scenarios such as [language negotiation](https://docs.google.com/spreadsheets/d/13x2S_xhGCD8ArUz9MiFPMSxCwrIVsbe1vb3Va35vGc8/edit#gid=0).

Finally, implementing and maintaining a core component with great performance characteristics, test coverage, and documentation allows us to establish high-quality culture within the project and expect that other components match it.

### Plural Rules

Pluralization is similar to Locale in that it is a core operation required by many other components, it’s a lean module and an ECMA-402 component.

As such it carries similar value and justification for inclusion in 1.0 scope.

### CodePointSet

Unicode Properties are required for many low level components including a number of our business and strategic alignments.

By providing a high quality API backed by a powerful data management solution, ICU4X becomes attractive for segmentation, collation, and unicode regular expression targets which we already see interest from.

Unicode Properties API is also critical for our ability to harness the dynamic Rust community as the GUI/layout needs arise.

### Segmentation

Segmentation aligns with business needs, allowing us to attract Mozilla investment and in return getting ability to test our model in production supplying the needs of both Gecko layout engine and ECMA-402 component.

The segmentation API carries also similar strategic value for the project as the CodePointSet as an attractor for the Rust community, but with a higher-level API allows us to validate performance claims in a performance-critical part of all layout needs which also serves our goal of attracting corporate interest in the project.

### DateTimeFormat

Date and Time formatting are the most common high-level requests out of internationalization APIs and a core part of ECMA-402.

On the technical side, this API allows us to validate our value proposition with a high-level, complex, highly demanded API. We have a chance to explore our modularization capabilities, high-volume data management, and non-trivial algorithmic requirements.

On the business side, we get a chance to supply the most common user needs, and cater to the Internationalization and Rust communities.

### NumberFormat

Number formatting represents a blend of low-level foundational APIs such as Locale, Plural Rules and CodePointSet, with high-level APIs such as DateTimeFormat.

It is required for DateTimeFormat, and will exercise our ability to maintain modularity and lean profile of our APIs, while building a large feature set that [ECMA-402 NumberFormat](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/NumberFormat) carries.

### FFI/WASM

FFI poses a lot of technical challenges that we have to overcome to validate the business value of ICU4X being a solution to the high-maintenance cost of keeping separate implementations for many target environments.

We’ll need to validate the ability to produce bindings to other programming languages and technically prove that the DataProvider model can fit those requirements.

## Stretch Goals

With the above listed components, ICU4X 1.0 will provide a comprehensive internationalization solution and validate its value proposition.

Components listed below are worth considering for 1.0 as resources and timing permits, but will not be necessary to prove the product.

### Collator

Collation is one of the core [ECMA-402 components](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/Collator), and builds on top of the Unicode Properties API which we’re investing in.

At the same time, technically it provides similar value to 1.0 scope as Segmenter, but without a clear business need making the core driver the ECMA-402 completeness.

As such, it is not required to prove the value of the project or evaluate its characteristics and market fit, which allows us to push it past 1.0.

### DurationFormat / ListFormat

Depending on the ability to modularize DateTimeFormat, Duration or List format may serve as high-level, simple targets for FFI validation.  
If a subset of DateTimeFormat is used, then the business and technical needs for such components are lowered and all such APIs can be implemented post 1.0.


## Roadmap

For the current roadmap, please see [ICU4X 1.0 Roadmap](./roadmap.md).
