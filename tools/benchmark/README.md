# Benchmark Tools

This folder contains tools for benchmarking examples.

## icu_benchmark_memory

This is an internal tool to collect dhat-rs instrumentation into an example file, run the example,
and collect the memory information. When running in CI, this data will then be collected into a
chart to allow seeing how memory changes over time.

### Usage

```sh
# Show help for the command.
cargo run --package icu_benchmark_memory

# Run memory benchmarks on examples "filter_langids" and "work_log"
cargo run --package icu_benchmark_memory -- icu_locid/filter_langids icu_datetime/work_log

# In CI, include the OS, and the data will be saved out per-OS.
cargo run --package icu_benchmark_memory -- --os macos-latest icu_datetime/work_log
```

dhat-rs outputs a summary to stderr, plus a `dhat-heap.json` file that can be viewed in the
[dhat viewer](https://nnethercote.github.io/dh_view/dh_view.html). The summary is collected by this
tool into an `.ndjson` located in the `benchmarks/memory` folder. This is the file that is read by
the benchmarking action in CI.

### Packages used:

* [dhat-rs](https://github.com/nnethercote/dhat-rs) for instrumentation.
* [benchmarking action](https://github.com/gregtatum/github-action-benchmark) – This is a fork that allows collecting [ndjson](http://ndjson.org/) benchmark data.

## Benchmark Macros

The macros contain helpers for examples so that they can be instrumented them tersely.
