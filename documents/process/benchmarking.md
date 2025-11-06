# Performance Benchmarking

ICU4X Benchmark Guideline
=========================

ICU4X is aiming to be a modular, fast and slim set of internationalization components indented
for use from many environments and ecosystems.
In order to achieve those goals, ICU4X has established a strong culture of measuring various
aspects of its performance over time in order to make informed decisions and notice regressions early.

At the moment we aim to measure four primary classes of benchmarks:

1) Performance
2) Memory utilization
3) Binary and data size
4) Code coverage

## Performance

Performance is primarily measured via a network of microbenchmarks located in each component
in its `benches/` directory.
Those benchmarks, running via the `criterion` statistical framework, are intended to give a reasonable
approximation of how the component should behave in the wild.

### Overview Benchmarks

Each component provides one or more tests with a `/overview` postfix, which can be filtered for using
`cargo bench overview`. Those benchmarks are intended to be run often in the change/test development cycle and give a good
overview of how the changes affect performance of the module in the most common scenarios.

This overview should be composed of no more than 5 tests (for a total of 1min per execution) and single iteration
should aim to be around or slightly above 1 microseconds and below 1ms on an average laptop to give
high p95 and a low standard deviation.

An example of a high-level overview from `PluralRules` component would be:
- `plurals/operands/overview` - overview of the `PluralOperands` creation cost
- `plurals/parser/overview` - overview of the parser cost
- `plurals/pluralrules/overview` - overview of the construction and common scenario usage of the main API

The naming convention is to start with the name of the crate (this helps when benchmarks are run from the top level across
multiple crates), then name of the module and then the name of the test, in this case `/overview`.

### Detailed Benchmarks

If more detailed benchmarks drilling into various facets of the component are needed, they should be put
in the same benchmark naming scheme, without the `overview` keyword.

For example:
  - `plurals/parser/lexing`
  - `plurals/operands/isize`
  - `plurals/operands/usize`
  - `plurals/operands/string`
  - `plurals/operands/fixed_Decimal`
  - `plurals/pluralrules/construct`
  - `plurals/pluralrules/select`

This allows a user to incorporate `cargo bench` into their change->test->change cycle, while allowing them
to drill into a more detailed view of the performance impact if needed by calling `cargo bench operands`.

### Wall time vs. Count of instructions

At the moment our microbenchmarks are using `wall-time` measurements which are prone to high stdev and will vary
from machine to machine and may be impacted by background load, CPU profiler and other external factors.

In the future we'd like to switch to measuring count of instructions as a more reliable measurement of performance
impact.

### Quasi "real-world" benchmarks

On top of micro-benchmarks, we will use small stand-alone binaries performing quasi real-world tasks
to compare our implementation against reference ICU4C equivalents, and between different ICU4C outputs (such
as WebAssembly, Dart, JS etc.).

## Memory

Memory benchmarks are not ready yet.

For memory benchmarks, we will measure two main concepts - Peak Memory Utilization and Data/Struct Allocation

### Peak Memory Utilization

We will attempt to instrument example binaries to collect peak memory usage and observe the utilization over time.

### Allocation

In many scenarios we can expect a certain instances of our structures to remain allocated in memory.

Examples of such are lists of locales, or total memory cost of data allocated on a `DateTimeFormat` instance.

We will measure that memory usage over time.

### Binary and Data Size

For binaries, we'll take apps in the `example/` directory of each component and instrument an optimized
builds and collect the total size of a static binary over time.

For data size, we will generate production ICU4X data and measure total size of the payload.

## Coverage

In order to ensure that our data driven testing provides sufficient coverage of our codepaths,
we'll want to run our instrumentation against only data-driven tests (as opposed to documentation tests,
in-module unit tests and imperative integration tests), and verify that the code coverage remains high.

We don't have an exact bar set yet for how high we want it to be, but we're aiming for total test coverage
to be above 90% and most of it should be covered by data driven tests.
