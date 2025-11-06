# Rust Date and Time API Request

In ICU4X 0.1 we introduce a `DateTimeFormat` API for formatting date and time into human readable formats. That API is shaped after [`ICU4C`](https://unicode-org.github.io/icu/userguide/format_parse/datetime/), [`ICU4J`](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/text/DateFormat.html) and [`ECMA-402`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat).

In order to clearly separate the concerns, we are leaving a challenging task of designing a complete solution for date and time management to the Rust community, and we can only formulate the request for what API ICU4X will need to format such structure.

We have prepared [a design doc](https://github.com/unicode-org/icu4x/blob/main/documents/datetime-input.md) discussing the data we need in ICU4X DateTimeFormat in order to produce properly localized strings.

## Separation of Concerns

We want to help the community understand the intended separation of concerns -- date and time management, including date and time arithmetic, should not be overlapped with formatting of the data.

Both tasks are very challenging and the history of Software Industry is filled with half-finished or badly designed APIs that attempted to "start small" and "scale" - a model which unfortunately doesn't work well for either of the two tasks.

We believe ICU4X DateTimeFormat will handle the formatting portion correctly with ergonomics, performance and quality backed by [`Unicode`](https://home.unicode.org/) and [`CLDR`](https://cldr.unicode.org/) projects.

In ICU4X, we also intend to implement a low-level, performance-focused API for calendrical calculations, enabling the conversion of dates between calendar systems. However, we do not intend to design a high-level API for this step; we intend to leave the ergonomic datetime arithmetic API to the community.

## Date and Time Management

There are surprisingly few examples of good date and time APIs in the industry that we feel confident to recommend. We strongly believe that the history of JavaScript [`Date`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date), C++ [`std::chrono`](https://en.cppreference.com/w/cpp/chrono) or Java [`LocalDateTime`](https://docs.oracle.com/javase/tutorial/datetime/iso/datetime.html) and a long history of shortcomings and limitations of them, particularly in relation to i18n, serve as a cautionary tale for anyone aiming to design a reference one for Rust.

In particular, we believe that correct date and time management API must include `calendar` and `time zone`.
It is our experience that such functionality is incredibly hard to retrofit into an existing API. Therefore it is our belief that such functionality must be planned very early in the API design.
We do not see any crate in the current ecosystem that covers those two features.

## Temporal

One example of an API that is being meticulously designed to handle the scope correctly is [EcmaScript Temporal Proposal](https://tc39.es/proposal-temporal/docs/index.html). We encourage everyone interested in the topic to read through the [Design Doc](https://github.com/tc39/proposal-temporal/blob/main/docs/calendar-draft.md) for it.

It is our hope that we can collaborate together to provide a complete date and time solution for management and formatting together.
