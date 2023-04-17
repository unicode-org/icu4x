# Release Process

For each release, we will fill a new Milestone, and an Issue with a Checklist.

Example [Milestone](https://github.com/unicode-org/icu4x/milestone/5) and [Issue](https://github.com/unicode-org/icu4x/issues/204#issuecomment-670819532).

Over the last month before the release target date, we'll discuss the checklist and milestone progress in our weekly calls, and fine tune timing and scope.

Once the release is complete, the assigned release driver will:

* Run `cargo +nightly fmt -- --config=format_code_in_doc_comments=true --config=doc_comment_code_block_width=80` to prettify our docs
* Run `cargo update` for each `Cargo.lock` file to update our CI to freshest dependencies
* Verify that the milestone and checklist are complete
* Verify with component owners that they're ready for release
* Verify that `ffi/diplomat` depends on a released (not Git) version of Diplomat. Get it published (ask manishearth or sffc) otherwise.
* Verify that we have acceptable FFI coverage (should be a part of the checklist issue)
  * Verify that `ffi/diplomat/tests/missing_apis.txt` is empty. If not, check with the team that we are okay punting these to the next release.
* Take a bird-eye view at:
  * READMEs
  * Documentation
  * Coverage
  * Performance / Memory / Size benchmarks
  * Cargo.toml files
    * Cargo.toml files need to specify versions for each entry in `dependencies`.
    * Cargo.toml should not specify versions for each entry in `dev-dependencies`.
    * Ensure that any new packages have suitable version numbers.
    * Ensure that the Cargo.toml file includes a set of fields consistent with other ICU4X packages.
* `cargo workspaces version --no-git-push --no-git-tag` to bump the version numbers
  * This will only update crates that have changed, and will ask you which version number to bump for each crate
  * You can use commands like `git log icu@1.0.0..@ -- components/plurals/src/` and `cargo public-api -p icu_list diff 1.0.0` to figure out whether to do a major, minor, or patch release
  * Get this reviewed and checked in before continuing
* Use `cargo workspaces publish --from-git` to automatically publish the crates in the correct order
  * Add `icu4x-release` group as owners to each new component you're publishing
    * `cargo owner -a github:unicode-org:icu4x-release`
* Verify that the tutorial crates work on the newly released crates.io sources by running `cargo make test-cargo-tutorial-cratesio`
  * If there are any errors, please fix them before advertising the release
* [Tag the Release](https://github.com/unicode-org/icu4x/releases)
* Announce the release to public
  * Send an email to [icu4x-announce](https://groups.google.com/u/0/a/unicode.org/g/icu4x-announce)


## Publishing utils

Our `utils/` crates do not follow the same versioning scheme as the rest of ICU4X, and may experience more frequent releases.

While code may compile using our local path dependencies, when publishing we must ensure that it will pull in the correct published version of a utils crate.

In general, if you ever cut a new release of a `utils/` crate, all `icu4x` crates depending on new behavior should have their `Cargo.toml` updated to the latest version. Typically, it is more convenient to update _all_ crates that depend on the utility, not just the ones that require the new behavior. If your new release introduces behavior that is not relied upon by any of our crates (e.g. some error formatting code was improved), it is fine to cut a release of the crate

When cutting new ICU4X releases, make sure all utilities with changes have had a new release containing those changes. To do so, go through the `utils/` folder and check the history of each crate since the last version bump. Bear in mind that some folders like `yoke/` contain multiple crates (`yoke/derive/`), and to keep derive-crates' versions in sync with their crates.

If there are no changes, ensure that the current version of the crate is the version in use in ICU4X's components. If not, make sure that ICU4X is not relying on any features since the release that is in use. In case of such reliance, update the version in use in the ICU4X components.

If there are changes, go through the changes and determine if they are breaking or not. For breaking changes, perform a breaking version bump (`x.y.z` to `x+1.0.0`, and `0.x.y` to `0.x+1.0`) and update all of ICU4X's components to use the new version.

For non breaking changes, perform a non-breaking version bump (`x.y.z` to `x.y.z+1` or `x.y+1.0` based on the size of the changes; `0.x.y` to `0.x.y+1`). Then, determine if the introduced functionality is being relied upon by ICU4X (assume it is if this is tricky to determine). If it is, update the version in use by the ICU4X components, otherwise it is fine to not do so.

This can all be done in a separate PR to chunk out the work but there should be no changes to utils between this PR landing and the overall ICU4X version bump. After landing the PR, as usual, `cargo publish` should be run on the updated utils.
