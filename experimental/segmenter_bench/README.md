# ICU4C/ICU4X Segmenter Benchmarks

This directory contains code to compare ICU4C BreakIterator with ICU4X Segmenter.

## Corpus

I use the UDHR corpus for testing:

```bash
$ wget https://unicode.org/udhr/assemblies/udhr_txt.zip
$ unzip udhr_txt.zip
$ export CORPUS=$PWD/udhr_txt/udhr_deu_1996.txt
```

## Running ICU4X

Build it:

```bash
$ cd experimental/segmenter_bench/rs
$ cargo build --release
```

Run it:

```bash
$ time target/release/run_icu4x_segmenter < $CORPUS 1> breakpoints.txt
```

## Running ICU4C

First, clone and build ICU4C:

```bash
$ git clone https://github.com/unicode-org/icu.git
$ cd icu/icu4c/source
$ export ICU4C_SRC=$PWD
$ ./runConfigureICU Linux --enable-static
$ make -j6
```

Back in this project, build the test runner:

```bash
$ cd experimental/segmenter_bench/cpp
$ make ICU4C_SRC=$ICU4C_SRC
```

And run it:

```bash
$ time ./run_icu_breakiterator < $CORPUS 1> breakpoints.txt
```
