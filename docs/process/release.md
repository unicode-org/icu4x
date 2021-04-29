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
* [Tag the Release](https://github.com/unicode-org/icu4x/releases)
* `cargo publish` each component and meta component
* Add `icu4x-release` group as owners to each new component you're publishing
  * `cargo owner -a github:unicode-org:icu4x-release`
* Announce the release to public
