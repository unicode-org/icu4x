# Rust version policy


## For ICU4X users

Each ICU4X release and its patches guarantee a minimum supported rust version (MSRV) of four Rust versions before the release, and attempt an MSRV of six Rust versions. For example, if today's current Rust stable version is 1.70, and we release ICU4X 1.3, then we guarantee that ICU4X 1.3 will have an MSRV of 1.66 or earlier. We will *attempt* an MSRV of 1.64.

The MSRV shall be included in the `rust-version` Cargo toml key, resulting in an error if used with an insufficiently updated toolchain.

Furthermore, we also try to guarantee some forward compatibility with future Rust versions. Rust's [stability policy] dictates that old Rust code may always successfully compile on newer compilers, however there are a couple caveats: Rust is allowed to break things to perform soundness fixes, and inference/imports may break in the case of ambiguities that get introduced as new APIs are added. 

While we hope that old ICU4X versions will always compile, this is not always going to be the case, and we want to ensure that one is not forced to upgrade ICU4X because their toolchain is getting upgraded. We shall strive to ensure old ICU4X versions continue to compile for at least 270 days, by working upstream with the Rust project on mitigations as breaking changes come down the pipeline, or by releasing patch releases if we fail to mitigate the issue in upstream Rust. Concretely, this means that a lockfile created with the current latest release will continue to work for 270 days, at most requiring patch release updates. This includes lockfiles created a day before a new release; in other words the actual supported lifetime of a release is 270 days plus the number of days before the next release. Please file bugs if a release fails to compile on a newer compiler (even if it's more than 270 days old), we would like to hear about it.

The main ICU4X library does not use nightly; however nightly is required for building our FreeRTOS port since it involves building a no_std binary. We shall attempt to keep such ports (and any future nightly-only ICU4X features) building on nightly compilers in a range of 3-6 months in the past and the future, however we cannot make concrete guarantees here.

 
## For ICU4X developers

Currently ICU4X pins a stable Rust version [in the `rust-toolchain` file](https://github.com/unicode-org/icu4x/blob/main/rust-toolchain), and additionally uses specific nightlies for [WASM tests](https://github.com/unicode-org/icu4x/blob/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54/Makefile.toml#L158) and [coverage](https://github.com/unicode-org/icu4x/blob/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54/.github/workflows/build-test.yml#L332). Furthermore, we enforce the current MSRV as a separate CI job.

One may update the MSRV to a Rust version that is 6 or more versions before the current stable (or the stable version that shall exist at the time of the next ICU4X release if it is planned) without needing any discussion. One may not update the MSRV to a Rust version that is not at least 4 versions before current stable.

For updating the MSRV to a Rust version that is between 4 and 6 versions the ICU4X developer must make a per-case argument as to why this update is beneficial to the project. Some example arguments may include that it serves a client need, or fixes a soundness issue.

When the nightly cronjob CI fails, ICU4X developers must fix it and ensure it does not represent a breakage that will affect users on older ICU4X versions. If it does, please work with upstream Rust on mitigation strategies, and if upstream Rust is unable to find a strategy that works, please perform patch releases in line with the policy laid out above.


 [stability-policy]: https://rust-lang.github.io/rfcs/1122-language-semver.html