# Contributing

See the file called [LICENSE](LICENSE) for terms applying to your contribution.

## Importing code

When importing pre-existing code, please observe the following:

### Rust crates

When importing existing (Apache-2.0 OR MIT)-licensed code, if the source files have license headers that refer to particular file locations that are no longer correct, add a comment under the license header saying that the licensing information has moved to the top-level LICENSE file. Copy incoming MIT license copyright notices from the imported code into the top-level LICENSE file.

Use the `license-file` key in `Cargo.toml` to refer to the LICENSE file.

### Code from ICU4C/J

When porting code from ICU4C or ICU4J, indicate in source code comments that a piece of code is ported under its original license. If you port code that is under a [third-party license](https://github.com/unicode-org/icu/blob/d95621c57f2becc1efd1be1d5c914624a715dac0/icu4c/LICENSE#L78-L414) in ICU4C/J as of the linked revision of the ICU4C LICENSE file and whose license isn't yet in the ICU4X LICENSE file, add a note about the part of code and the third-party license to the exception list in the LICENSE file, include the third-party license at the end of the LICENSE file with the title of the license, and in the code use comments to indicate that the code is under the particular third-party license.

### Other cases

Please discuss first.
