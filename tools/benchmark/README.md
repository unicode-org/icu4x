# memory_bench

This is an internal tool to inject the necessary dhat-rs instrumentation into an
example file, run the example, and collect the memory information. When running in CI,
this data will then be collected into a chart to allow seeing how memory changes over
time.

## Packages used:

* [dhat-rs](https://github.com/nnethercote/dhat-rs) for instrumentation.
* [benchmarking action](https://github.com/gregtatum/github-action-benchmark) – This is a fork that allows collecting [ndjson](http://ndjson.org/) benchmark data.

## Usage

This tool can be run on multiple examples in a single component. So for instance
take the following examples:

```
components/locid/examples/filter_langids.rs
components/locid/examples/syntatically_canonicalize_locales.rs
```

The memory instrumentation can be run locally like this:

```
cargo run --package memory_bench -- macos-latest components/locid filter_langids syntatically_canonicalize_locales
```

dhat-rs outputs a summary to stderr, plus a `dhat-heap.json` file that can be viewed in the [dhat viewer](https://gregtatum.github.io/dhat-viewer/dh_view.html). The summary is collected by this tool into an `.ndjson` located in the `benchmarks` folder. This is the file that is read by the benchmarking action in CI.
