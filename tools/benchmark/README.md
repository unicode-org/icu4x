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

There is also some experimental support for viewing these profiles in a [deploy preview for the
Firefox Profiler](https://deploy-preview-3128--perf-html.netlify.app/). Drag the `dhat-heap.json`
file into the UI, and the memory can be analyzed using the call tree and flame graph.
[PR #3128](https://github.com/firefox-devtools/profiler/pull/3128) is the tracking issue for
official support.

### Packages used:

* [dhat-rs](https://github.com/nnethercote/dhat-rs) for instrumentation.
* [benchmarking action](https://github.com/gregtatum/github-action-benchmark) – This is a fork that allows collecting [ndjson](http://ndjson.org/) benchmark data.

## Valgrind / Callgrind

ICU4X examples are also instrumented to produce Callgrind artifacts demonstrating the call graph and instruction reads.

To generate callgrind artifacts for all examples, run from the root directory:

```bash
$ cargo make valgrind
12410 Ir: filter_langids 
3912 Ir: permyriad 
84194 Ir: language_names_hash_map 
209698 Ir: syntatically_canonicalize_locales 
3946 Ir: language_names_lite_map 
24874 Ir: unicode_bmp_blocks_selector 
201246 Ir: writeable_message 
153702 Ir: code_line_diff 
794477 Ir: work_log 
785178 Ir: tui 
123768 Ir: elevator_floors 
105762 Ir: unread_emails 
```

To view a call graph, use the `kcachegrind` or `gprof2dot` tools, as suggested [here](https://stackoverflow.com/questions/9279144/interpreting-callgrind-data). For example:

```bash
$ kcachegrind benchmarks/valgrind/code_line_diff.out
```

To generate a png file:

```bash
$ pip install gprof2dot
$ gprof2dot --format=callgrind --output=out.dot benchmarks/valgrind/tui.out
$ dot -Tpng out.dot -o graph.png
$ open graph.png
```

## Benchmark Macros

The macros contain helpers for examples so that they can be instrumented them tersely.

### Feature: benchmark_memory

This feature enables the dhat-rs instrumentation described above. To enable it for any example in the ICU4X tree, add `--features icu_benchmark_macros/benchmark_memory`. For example, run this from the top level:

```bash
$ cargo run --example code_line_diff --features icu_benchmark_macros/benchmark_memory
Added/Removed: -৫০/+৭২
Added/Removed: ০/+৩,৭৫০
Added/Removed: -১,২০১/০
Added/Removed: -৯,৮৭৬/+৫,৪৩২
Added/Removed: -৫০,০০,০০০/+৩০,০০,০০০
dhat: Total:     13,793 bytes in 69 blocks
dhat: At t-gmax: 8,973 bytes in 6 blocks
dhat: At t-end:  1,112 bytes in 3 blocks
dhat: The data in dhat-heap.json is viewable with dhat/dh_view.html
```

### Feature: rust_global_allocator

This feature replaces the default system allocator with `dlmalloc`, the same Rust-based allocator used in the `wasm32-unknown-unknown` build target.  This results in more reliable benchmarks by removing the platform-dependent allocator.

This feature is used by the `cargo make valgrind` build task.

## icu_benchmark_binsize

This is a continuous integration tool to monitor the size of ICU4X binaries.
It is invoked by GitHub Action for each PR, measures the size of the ICU4X demo
files compiled to wasm (WebAssembly) format, and stores the result in a
branch of the repository for subsequent display.

### Usage and Output

```sh
# Prerequisite: build wasm binaries.
cargo make wasm-examples

# Execute binsize benchmark
cargo run --package icu_benchmark_binsize -- wasmpkg/wasm-opt
```

Benchmark output is written in ndjson format, e.g.
`{"biggerIsBetter":false,"name":"simple","unit":"bytes","value":909161}`
for binary `simple.wasm`.

### Packages used

[benchmarking action](https://github.com/gregtatum/github-action-benchmark) – This is a fork that allows collecting
[ndjson](http://ndjson.org/) benchmark data.
