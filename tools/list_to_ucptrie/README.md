# CodePointTrie Builder Tool

This tool is intended as a temporary mechanism for building CodePointTrie data until ICU4X supports it natively in Rust: [#1837](https://github.com/unicode-org/icu4x/issues/1837).

## Installation

Get a local checkout of ICU4C and build it:

```bash
$ cd icu4c/source
$ ./runConfigureICU Linux --enable-static
$ make -j6
```

In this directory, build the tool:

```bash
$ make ICU4C_SRC=/path/to/icu4c/source
```

You now have a binary `list_to_ucptrie` with ICU4C built-in that you can run.

## Usage

`list_to_ucptrie` takes line-separated integers in STDIN, corresponding to the trie values for code points 0, 1, 2, and so on. When it receives an EOF (`^D`), it prints helpful information to STDERR and dumps the CodePointTrie TOML to STDOUT.

It takes three positional arguments:

1. Default value for code points (integer)
2. Error value for out-of-range code points (integer)
3. TrieType: `fast` or `small`

## Example: Manual Invocation

Invoke the command:

```bash
$ ./list_to_ucptrie 0 0 small
```

Type the following in STDIN:

```
22
33
44
```

Press `^D` and watch the magic occur:

```
Saved values for code points up to 3
index = [
  0,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,
  0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,
  0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,
  0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40,0x40
]
data_8 = [
  0x16,0x21,0x2c,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0
]
indexLength = 64
dataLength = 128
highStart = 0x200
shifted12HighStart = 0x1
type = 1
valueWidth = 2
index3NullOffset = 0x7fff
dataNullOffset = 0x40
nullValue = 0x0
```

## Example: From Value File

To re-generate code point tries in datagen input, run from the icu4x directory (e.g. for grapheme):

```bash
$ ./tools/list_to_ucptrie/list_to_ucptrie 0 0 small < provider/datagen/data/grapheme_cptrie.txt > provider/datagen/data/grapheme_cptrie.toml
```
