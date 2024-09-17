# Release Process

For each release, we will fill a new Milestone, and an Issue with a Checklist.

Example [Milestone](https://github.com/unicode-org/icu4x/milestone/5) and [Issue](https://github.com/unicode-org/icu4x/issues/204#issuecomment-670819532).

Over the last month before the release target date, we'll discuss the checklist and milestone progress in our weekly calls, and fine tune timing and scope.


## Pre-release checklist

This is a checklist of things that should be done in the weeks leading to the release.

* [ ] Verify that the milestone and checklist are complete
* [ ] Verify with component owners that they're ready for release
* [ ] Take a bird-eye view at:
  * [ ] READMEs
  * [ ] Documentation
  * [ ] Coverage
  * [ ] Performance / Memory / Size benchmarks
  * [ ] Cargo.toml files
    * [ ] All dependencies from the workspace should use `workspace = true` rather than their version number or path 
    * [ ] Cargo.toml files need to specify versions for each non-workspace entry in `dependencies`, or use `workspace = true`.
    * [ ] Ensure that any new packages have version number `0.0.0`, this will making bumping during the release easier.
    * [ ] Ensure that the Cargo.toml file includes a set of fields consistent with other ICU4X packages.
        * These should mostly use workspace inheritance
* [ ] Run `cargo +nightly fmt -- --config=format_code_in_doc_comments=true --config=doc_comment_code_block_width=80` to prettify our docs
* [ ] Run `cargo update` for each `Cargo.lock` file to update our CI to freshest dependencies
* [ ] Go through `ffi/capi/tests/missing_apis.txt` and verify that it is empty. If it is not, component owners should either add FFI APIs, add `rust_link` annotations, or allowlist the relevant APIs as having been punted to the future
* [ ] Verify that `ffi/capi` depends on a released (not Git) version of Diplomat. Get it published (ask manishearth or sffc) otherwise.
* [ ] Get all contributors to complete the changelog (see below)
* [ ] Draft the text for the GitHub release. This text will be sent to GitHub subscribers and can also be used for the mailing list email and blog post.
* [ ] Prepare a PR to update tutorials using the upcoming release. The PR should pass `cargo make test-tutorials-local`, but can fail `cargo make test-tutorials-cratesio` prior to release

## Release steps

Once the release checklist is complete, the assigned release driver will perform the following steps, in order:

* Land the changelog (see below)
  * Note: Do this _before_ tagging the release so that it is included in the tag
* Go through the prerelease checklist again, ensuring that no problems were reintroduced in the PRs that landed since the opening of the checklist. (Things like doc prettification will likely need to be rerun!)
* Release utils (see section below). Get the version bumps reviewed and checked in before releasing.
* Update ICU4X package versions as needed. Most of this can be done by updating `workspace.package` in the root `Cargo.toml` and the `workspace.dependencies` entries there. Some `icu_*` crates do not follow the ICU4X versioning scheme like `icu_codepointtrie_builder` or experimental crates.
* Get this reviewed and checked in before continuing
* Publish each `icu_*` crate
  * Use `cargo workspaces publish --from-git` to automatically publish the crates in the correct order if you would like
  * Add `icu4x-release` group as owners to each new component you're publishing
    * `cargo owner -a github:unicode-org:icu4x-release`
* Merge the tutorials PR. `cargo make test-tuturials-cratesio` should now pass
  * If there are any errors, please fix them before advertising the release
* [Tag the Release](https://github.com/unicode-org/icu4x/releases) with the text drafted above
* Create a branch named `release/x.y` on the release tag and push it to the upstream
* Announce the release to public
  * (All releases) Send an email to [icu4x-announce](https://groups.google.com/u/0/a/unicode.org/g/icu4x-announce)
  * (All releases) Blog post on Unicode blog
  * (Major releases only) Submit to This Week In Rust
* Keep the main branch relatively stable for 7-14 days following the release to make things easier in case a patch release is needed.
  * It's okay to land smaller or incremental changes, but avoid breaking changes during this period.

## Patch Releases

The ICU4X TC may decide to make a patch release of an ICU4X component on an old release stream, such as to fix a regression in behavior. To make a patch release:

* Fix the issue on the main branch. Get it reviewed and landed.
  * Include an update to the changelog.
  * If possible, avoid mixing functional changes with generated files (e.g. data or FFI) in the commit that lands on the main branch.
* For util crates (and other crates not versioned with ICU4X), update their `Cargo.toml`s on `main` to reflect the version you wish to publish, to simplify things for people making ICU4X major/minor releases in the future. Do NOT publish from `main`.
* Check out the `release/x.y` branch. On this branch:
  * Cherry-pick the functional change from the main branch
  * Cherry-pick the changelog update if it was a separate commit
  * Land re-generated code or data
  * Land a commit updating the version number of the component that needs the patch
  * Have a team member review the branch before continuing
* Release the updated components from the tip of `release/x.y`
* Create and push a tag of the format `ind/icu_collator@1.3.3` (for icu_collator patch version 1.3.3)
  * It is not necessary to create `ind/icu_collator_data@1.3.3` so long as the component has a tag

## Publishing utils

Our `utils/` crates do not follow the same versioning scheme as the rest of ICU4X, and may experience more frequent releases.

While code may compile using our local path dependencies, when publishing we must ensure that it will pull in the correct published version of a utils crate.

In general, if you ever cut a new release of a `utils/` crate, all `icu4x` crates depending on new behavior should have their `Cargo.toml` updated to the latest version. Typically, it is more convenient to update _all_ crates that depend on the utility, not just the ones that require the new behavior. If your new release introduces behavior that is not relied upon by any of our crates (e.g. some error formatting code was improved), it is fine to cut a release of the crate

When cutting new ICU4X releases, make sure all utilities with changes have had a new release containing those changes. To do so, go through the `utils/` folder and check the history of each crate since the last version bump. Bear in mind that some folders like `yoke/` contain multiple crates (`yoke/derive/`), and to keep derive-crates' versions in sync with their crates.

If there are no changes, ensure that the current version of the crate is the version in use in ICU4X's components. If not, make sure that ICU4X is not relying on any features since the release that is in use. In case of such reliance, update the version in use in the ICU4X components.

If there are changes, go through the changes and determine if they are breaking or not. For breaking changes, perform a breaking version bump (`x.y.z` to `x+1.0.0`, and `0.x.y` to `0.x+1.0`) and update all of ICU4X's components to use the new version.

For non breaking changes, perform a non-breaking version bump (`x.y.z` to `x.y.z+1` or `x.y+1.0` based on the size of the changes; `0.x.y` to `0.x.y+1`). Then, determine if the introduced functionality is being relied upon by ICU4X (assume it is if this is tricky to determine). If it is, update the version in use by the ICU4X components, otherwise it is fine to not do so.

This can all be done in a separate PR to chunk out the work but there should be no changes to utils between this PR landing and the overall ICU4X version bump. After landing the PR, as usual, `cargo publish` should be run on the updated utils.

## Writing a changelog

In general, the *Unreleased* section of the changelog should be updated with each changelog-worthy PR. However, as this might be forgotten, before a release you should ping all major contributors, and ask them to complete their parts of the changelog. Before the release, rename the *Unreleased* section to the appropriate version.

Out-of-cycle changelogs should use a single entry for each individual crate released, e.g. something like this:

```markdown
- `databake`: 0.1.5
  - Fixed [#3356](https://github.com/unicode-org/icu4x/pull/3356), adding `allow` for clippy false-positives
```

