# Release Process

For each release, we will fill a new Milestone, and an Issue with a Checklist.

Example [Milestone](https://github.com/unicode-org/icu4x/milestone/5) and [Issue](https://github.com/unicode-org/icu4x/issues/204#issuecomment-670819532).

Over the last month before the release target date, we'll discuss the checklist and milestone progress in our weekly calls, and fine tune timing and scope.

Once the release is complete, the assigned release driver will:

* Verify that the milestone and checklist are complete
* Verify with component owners that they're ready for release
* Take a bird-eye view at:
  * READMEs
  * Documentation
  * Coverage
  * Performance / Memory / Size benchmarks
* Build a changelog for the release
  * You can use commands like `git log icu@0.4.1..@ -- components/plurals/src/` to figure out what has changed in each component since the last release
* [Tag the Release](https://github.com/unicode-org/icu4x/releases)
* `cargo publish` each `util/` as necessary (See [Publishing utils](#Publishing utils))
* `cargo publish` each component and meta component
* Add `icu4x-release` group as owners to each new component you're publishing
  * `cargo owner -a github:unicode-org:icu4x-release`
* Ensure that the steps in `docs/tutorials/intro.md` still work with updated version numbers
* Announce the release to public


## Publishing utils

Our `utils/` crates do not follow the same versioning scheme as the rest of ICU4X, and may experience more frequent releases.

While code may compile using our local path dependencies, when publishing we must ensure that it will pull in the correct published version of a utils crate.

In general, if you ever cut a new release of a `utils/` crate, all `icu4x` crates depending on new behavior should have their `Cargo.toml` updated to the latest version. Typically, it is more convenient to update _all_ crates that depend on the utility, not just the ones that require the new behavior. If your new release introduces behavior that is not relied upon by any of our crates (e.g. some error formatting code was improved), it is fine to cut a release of the crate

When cutting new ICU4X releases, make sure all utilities with changes have had a new release containing those changes. To do so, go through the `utils/` folder and check the history of each crate since the last version bump. Bear in mind that some folders like `yoke/` contain multiple crates (`yoke/derive/`).

If there are no changes, ensure that the current version of the crate is the version in use in ICU4X's components. If not, make sure that ICU4X is not relying on any features since the release that is in use. In case of such reliance, update the version in use in the ICU4X components.

If there are changes, go through the changes and determine if they are breaking or not. For breaking changes, perform a breaking version bump (`x.y.z` to `x+1.0.0`, and `0.x.y` to `0.x+1.0`) and update all of ICU4X's components to use the new version.

For non breaking changes, perform a non-breaking version bump (`x.y.z` to `x.y.z+1` or `x.y+1.0` based on the size of the changes; `0.x.y` to `0.x.y+1`). Then, determine if the introduced functionality is being relied upon by ICU4X (assume it is if this is tricky to determine). If it is, update the version in use by the ICU4X components, otherwise it is fine to not do so.

This can all be done in a separate PR to chunk out the work but there should be no changes to utils between this PR landing and the overall ICU4X version bump. After landing the PR, as usual, `cargo publish` should be run on the updated utils.
