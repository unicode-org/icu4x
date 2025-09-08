# Rust version policy


## For ICU4X users

Each ICU4X release and its patches guarantee a minimum supported rust version (MSRV) of four Rust versions before the release, and attempt an MSRV of six Rust versions. For example, if today's current Rust stable version is 1.70, and we release ICU4X x.y.0, then we guarantee that ICU4X x.y.z will have an MSRV of 1.66 or earlier. We will *attempt* an MSRV of 1.64.

The MSRV shall be included in the `rust-version` Cargo toml key, resulting in an error if used with an insufficiently updated toolchain.

Furthermore, we also try to guarantee some forward compatibility with future Rust versions. Rust's [stability policy] dictates that old Rust code may always successfully compile on newer compilers, however there are a couple caveats: Rust is allowed to break things to perform soundness fixes, and inference/imports may break in the case of ambiguities that get introduced as new APIs are added. 

While we hope that old ICU4X versions will always compile, this is not always going to be the case, and we want to ensure that one is not forced to upgrade ICU4X because their toolchain is getting upgraded. We shall strive to ensure old ICU4X versions continue to compile for at least 270 days, by working upstream with the Rust project on mitigations as breaking changes come down the pipeline, or by releasing patch releases if we fail to mitigate the issue in upstream Rust. Concretely, this means that a lockfile created with the current latest release will continue to work for 270 days, at most requiring patch release updates. This includes lockfiles created a day before a new release; in other words the actual supported lifetime of a release is 270 days plus the number of days before the next release. Please file bugs if a release fails to compile on a newer compiler (even if it's more than 270 days old), we would like to hear about it.

The main ICU4X library does not use nightly; however nightly is required for building our FreeRTOS port since it involves building a no_std binary. We shall attempt to keep such ports (and any future nightly-only ICU4X features) building on nightly compilers in a range of 3-6 months in the past and the future, however we cannot make concrete guarantees here.


### Utils crates with different MSRVs

Our "utils" crates (yoke, zerovec, zerofrom, etc) are not versioned alongside ICU4X: they may have major version updates between minor ICU4X releases, and they may choose to have minor version updates between major ICU4X releases.

By default, these crates have the same MSRV as their corresponding ICU4X release, however at times we may give them a lower MSRV for a wider range of compatability.

This is done on an as-requested basis and is best-effort.

Currently, `zerofrom` is the only crate with a lower MSRV.
 
## For ICU4X developers

Currently ICU4X pins a stable Rust version [in the `rust-toolchain` file](https://github.com/unicode-org/icu4x/blob/main/rust-toolchain), and additionally uses specific nightlies for [WASM tests](https://github.com/unicode-org/icu4x/blob/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54/Makefile.toml#L158) and [coverage](https://github.com/unicode-org/icu4x/blob/1f4a9505f21a6d5c9bb4833e0cf3fe969f734c54/.github/workflows/build-test.yml#L332). Furthermore, we enforce the current MSRV as a separate CI job.

One may update the MSRV to a Rust version that is 6 or more versions before the current stable (or the stable version that shall exist at the time of the next ICU4X release if it is planned) without needing additional approval by stating which features in that release are useful for ICU4X development. One may not update the MSRV to a Rust version that is not at least 4 versions before current stable.

For updating the MSRV to a Rust version that is between 4 and 6 versions the ICU4X developer must make a per-case argument as to why this update is beneficial to the project. Some example arguments may include that it serves a client need, or fixes a soundness issue.

When the nightly cronjob CI fails, ICU4X developers must fix it and ensure it does not represent a breakage that will affect users on older ICU4X versions. If it does, please work with upstream Rust on mitigation strategies, and if upstream Rust is unable to find a strategy that works, please perform patch releases in line with the policy laid out above.


 [stability-policy]: https://rust-lang.github.io/rfcs/1122-language-semver.html


### Pinned Rust nightly policy

ICU4X pins Rust Nightlies in CI for multiple purposes:

 - `diplomat-coverage/src/main.rs`: Running the diplomat-coverage task, which uses unstable Rustdoc JSON output, requiring synchronization between the Rust version and the `rustdoc_types` dependency.
 - `PINNED_CI_NIGHTLY`: Running various FFI CI jobs that need _some_ Rust nightly to use build-std or other nightly features, without becoming beholden to arbitrary
 - `LLVM_COMPATIBLE_NIGHTLY`: Usage of `-Clinker-plugin-lto` in size-optimized tests where Rust and Clang need to be using compatible LLVM versions.


 ICU4X also runs its entire CI against the latest nightly once a day, reporting errors to Slack. These errors help us catch upstream breakages early, but ICU4X does not have any particular SLO against these being fixed. We guarantee we build on the latest stable as detailed in the first section of this page.

 The first two nightlies here can be updated whenever necessary to whatever nightly works. It not necessary to keep the nightly versions the same, though it is ideal to try. The choice of these nightlies does not have much of a direct impact on users.


For users who wish to use ICU4X with `-Clinker-plugin-lto`, we have some guarantees about the nightly they can use it with (`LLVM_COMPATIBLE_NIGHTLY`, found in the makefiles under `tutorials/c-tiny`). We recommend usage of ICU4X with `-Clinker-plugin-lto`, on `LLVM_COMPATIBLE_NIGHTLY`, for size-constrained clients who have the option available. To ensure this option is readily available to ICU4X clients, we guarantee that `LLVM_COMPATIBLE_NIGHTLY` must work with "widely available LLVM" versions. This may further constrain our MSRV policy above in case `LLVM_COMPATIBLE_NIGHTLY` cannot be pinned to something that works with an otherwise-allowed MSRV.

One can test that a candidate `LLVM_COMPATIBLE_NIGHTLY` "works with" an LLVM version when the size test benchmarks (makefiles in `tutorials/c-tiny/`) run without erroring due to mismatched LLVM metadata. Note that Rust uses LLVM trunk, so it may happen that a Rust nightly is not compatible with _any_ released LLVM version. Some experimentation is often required to find a nightly/LLVM pair that works.


"widely available LLVM" here means that:

 - It MUST be available via apt on all currently-supported Ubuntu LTS
 - It MUST be available via apt on Debian `testing`
 - It MUST be available via `brew`
 - It MUST be available on Fedora via `yum`
 - It MUST be available on the GitHub Actions runners with `ubuntu-latest`
 - It SHOULD be available on the latest XCode
 - It SHOULD be available on ICU4X developer machines using nonstandard package management (more on this below)
 - It SHOULD be available on RHEL and latest Rocky Linux via `yum`
 
Here, SHOULD requirements can be overridden by TC approval, where the developer must make a per-use case argument as to why the upgrade is beneficial to the project.

More information on each of these bullet points, and how to verify them without needing that OS. below. Commands are for installing LLVM 18, for illustrative purposes.


#### Ubuntu

Policy: Must be available on the oldest still-supported Ubuntu LTS.

Command: `apt install llvm-18` on Ubuntu LTS.

Verification:  Package should be available on [oldest still-supported Ubuntu LTS][ubuntu-release]. Visit the page https://launchpad.net/ubuntu/+source/llvm-toolchain-18 (with the number replaced with the desired LLVM version), and scroll down to see if the oldest active Ubuntu LTS is there. [Currently][ubuntu-release], that is 20.04 Focal Fossa, starting April 2025 it will be Jammy Jellyfish (until April 2027).


 [ubuntu-release]: https://ubuntu.com/about/release-cycle

#### Debian

Policy: Must be available on Debian testing.

Command: `apt install llvm-18` on Debian testing.


Verification:  Visit the page https://packages.debian.org/search?keywords=llvm-18&searchon=names&suite=testing&section=all (with the number 18 replaced by the desired LLVM version) and ensure the package is listed under "Exact matches".

Note: "Debian stable" is typically _very_ old and is not what most Debian-based systems use. E.g. Debian stable is currently on Rust 1.63, which is more than two years old. Stable is intended to be extremely stable without even performing backwards-compatible upgrades.

#### Fedora

Policy: Must be available on latest stable Fedora

Command: `yum install llvm18`

Verification: Visit the page https://packages.fedoraproject.org/pkgs/llvm18/llvm18/ (with the number 18 replaced with desired LLVM version), and ensure that in the "Releases overview" table, there is an entry in the "Stable" column for "Fedora N" where N is a number. You can double check that N is the latest stable by visiting [this page](https://fedoraproject.org/).

#### XCode

Policy: Should be available on the latest XCode, but policy can be overridden by TC approval in cases where there is a strong case for a new feature.

Command: none


Verification: Look at the bottom of [XCode's Wikipedia page][xcode-wiki] and check that the latest released XCode has an LLVM version equal to or greater than the one being used.

 [xcode-wiki]: https://en.wikipedia.org/wiki/Xcode#Xcode_15.0_-_(since_visionOS_support)_2

#### Brew

Policy: Must be available via brew.

Command: `brew install llvm@18` on OSX.

Verification:  Visit https://formulae.brew.sh/formula/llvm#default and ensure the `llvm@N` package is listed, for version N.

#### RHEL / Rocky Linux

Policy: Should be available via `yum`, but policy can be overridden by TC approval in cases where there is a strong case for a new feature.

Command: `yum module install llvm-toolset` or `dnf install llvm-toolset` (RHEL 9)

Verification: Latest Rocky Linux (currently Rocky Linux 9) and RHEL should be listed on [this page](https://pkgs.org/search/?q=llvm-toolset) with an LLVM version that is equal to or greater than the desired LLVM version. It's okay if the exact LLVM version is not listed, `yum module install` allows installing older versions via explicit command.

#### GitHub Actions

Policy: Must be available on GitHub actions `ubuntu-latest` (i.e. ICU4X CI should be able to just `apt install` the LLVM)

Command `apt install llvm-18` on ICU4X CI.

Verification:  `ubuntu-latest` is always older than the oldest still-supported Ubuntu LTS, so this does not need to be verified separately. There are potential edge cases where GitHub's package repository mirrors are lagging behind where this policy basically means that we should wait a few days for things to smooth out. GitHub's precise runner image version can be looked up [here](https://github.com/actions/runner-images/blob/main/images/ubuntu/Ubuntu2404-Readme.md).

#### ICU4X developer machines

ICU4X developers may request additional criteria for their machines in case their machines use nonstandard package management, for example employer-managed machinees. These criteria will be listed below, and adding new criteria should require TC approval.

Policy: Should be available on ICU4X developer machines based on criteria below. Overriding this policy requires approval from affected developers.

Verification: Affected ICU4X developers are in charge of testing this: if developer A wishes to perform an LLVM upgrade to a version satisfying all of the above policies, they may do so without checking if it works on developer B's machine, but developer B is allowed to block or revert the PR if they have issues.

The current criteria are:

 - "Available on GLinux" (Used by Googler ICU4X developers): GLinux packages are based on Debian `testing` so this should almost always be true based on previous criteria, but there may be occasional cases where things lag behind for a week or two, in which case the Rust upgrade should be delayed til fixed.
