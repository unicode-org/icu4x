# Contributing to ICU4X Project

`ICU4X` is an open source project and welcomes everyone to participate.

In order to provide meaningful contributions, it is important to familiarize yourself with a set of documents which describe the [structure](./docs/charter.md) and [style](./docs/style-guide.md) used by the project.

The list of [open issues](triaging.md) represent the current focus points of the project, and the [help wanted](https://github.com/unicode-org/icu4x/issues?q=is%3Aissue+label%3A%22help+wanted%22+no%3Aassignee+) lists the issues which we don't have resources to work on right now, but would consider accepting if someone volunteered to work on them.

Issues are open to everyone to discuss and can be used to jump-start Pull Requests intended for the project.

In most cases, the first step is to find or file a new issue related to your planned contribution, discuss it, and once you received a feedback indicating that the pull request would be welcomed you can start working on it.

## Contributing a Pull Request

The first step is to fork the repository to your namespace and create a branch off of the `master` branch to work with.

That branch may end up containing one of more commits that are constituting the full scope of the pull request.

### Checklist

Each commit and pull request should follow the [style guide](./docs/style-guide.md) and pass all checks such as tests and `cargo fmt`. If the PR is adding any public API changes, we'd also like to ensure that full coverage of `cargo doc` is preserved and code coverage is above `90%`.

### Structure of commits in a Pull Request

Pull Request lifecycle is divided into two phases.

The first one is the work done to get the Pull Request ready for review.
The other is the review cycle.

#### Draft Phase

If the pull request is simple and short lived, it can be initialized with review request.
If the pull request is more complex and is being developed over time, it may be benefitial to start it in a `Draft` state.
This allows other contributors to monitor the progress and volunteer feedback while annotating that the pull request is not yet ready for review.

By the end of this phase, and right before review is requested, it is helpful for the reviewers to have a clean list of commits in the pull request.

In most cases, a single commit per pull request is enough.

Multiple commits should be used when the commit is too large and the scope of changes can be reduced by separating it into multiple commits which are logically self-contained.
Such commits do not have to pass tests in isolation, and need only to be meaningfully complete for the reviewer to benefit from reading, compared to reviewing all the changes at once.

#### Review Phase

Once the pull request is ready for review and passes all tests, the author can switch from draft to regular pull request.

At this point, the pull request will be triaged during the next triage session and reviewers will be assigned to it.

In this phase, any changes applied to the pull request should result in additive commits to it. This allows reviewers to see what changes have been made in result of their feedback and evaluate them.

### Merging

Every PR requires at least one review to be merged.

If the author has the editing rights to the repository merging should be performed by the author of the pull request. If the author wants to grant another team member rights to merge, they can state so in the PR comment.

If the pull request modifies code in one of the recognized components, one of the component owners should be on the reviewers list for the pull request. For the list of components and their owners, see [CODEOWNERS](CODEOWNERS).

## Review Model

Every project has its own code authoring and review culture which grows organically as the project matures and is a subject to change.
Below is the description of the model we try to follow, with exceptions when the common sense dictates otherwise.

### Review Types

Review can be of either **architectural** or **technical** value, and often is of both.

The PR author can specify, when requesting review, what kind of review they are asking for, from each reviewer, or even which area they'd like the reviewer to focus on (useful when distributing reviews).

### Reviewer Role

The reviewer is responsible for accepting a pull request only once they feel the current PR is ready to be merged even if their comments were not to be applied.

The *approve* can be set with pending review comments, if those comments don't affect whether the patch is ready to be merged (for example, they're stylistic suggestions).

The reviewer should communicate the nature of their review comments - specifically, between the three types: *"blocking"*, *"suggestion"*, and *"optional"*.
* **blocking** is when the reviewer considers the change to be unmergable and requires a new revision.
* **suggestion** is for when the reviewer considers the change to be suboptimal, but usable, and wants to defer the decision to the PR author, while stating their opinion.
* **optional** is for when the reviewer considers multiple options to be mostly comparable or tradeoffs, and wants to defer to the PR author for the final decision after bringing up a new option.

### Social Contract

`ICU4X` project focuses on a fairly hermetic domain of software internationalization, which requires prior knowledge of the domain.
With that in mind, most engineers working on patch authoring and reviews are expected to be senior enough to be trusted with the quality of their additions to the code.

For those reasons, we are primarily placing **trust** in pull request authors to write high quality, readable, tested, maintainable and well documented code.

The role of the **reviewer** in such model is more conservative and is reduced to verification of the code from a particular angle with minimal impact on the pull request.
Examples of such angle may be:

* How the PR fits into the component's public API
* Alignment of the PR with the goals and scope of the project
* Memory management of the code in pull request
* Test coverage, and sanity checks
* Use of CLDR, Unicode, Rust and other best practices
* Consistency with the ICU4X [style guide](./docs/style-guide.md)
* Use of I/O, data management, etc.

The pull request author is expected to evaluate what kind of review(s) they need to ensure the quality of their pull request.

An important piece of the reviewer's role is to correctly employ the three types of review comments (required, suggestion, or optional).

Lastly, the reviewer's role is to evaluate the stakeholders group and ensure that the review coverage is complete - if they review only portion of the PR, or if they see the need for additional stakeholders to be involved, they should add additional reviewers, or CC them into the issue, depending on what kind of involvement they expect (inform vs verify).


#### Mentorship scenario

The approach listed above describes the culture we aspire to based on the assumptions about the composition of the contributors group, and should be adjusted when the pull request author is new to the field and doesn't have the domain expertise.
In such cases, *mentorship model* should be used where a more senior engineer takes a role of a mentor.

### Reviewer Selection

When the PR author creates a new PR, they should consider three sources of reviewers and informed stakeholders:

* Owners and peers of the component they work with
* People involved in the preceeding conversation
* Recognized experts in the domain the PR operates in

The goal of the PR author is to find the subset of stakeholders that represent those three groups well. Depending on the scope and priority of the PR, the reviewers group size can be adjusted, with small PRs being sufficent for review by just one stakeholder, and larger PRs, or first-of-a-kind using a larger pool of reviewers.

### PR author and reviewers workflow

When the PR author submits the PR for review, and once they selected the initial group, each member of that group can chose to either review or resign from reviewing.
If they resign, their only duty is to ensure that there is a sufficient coverage of the review, and nominate their replacement if not.

After a round of review, if there are blocking issues or a reviewer kept themselves in the review queue without accepting a review, the PR author must update the PR and re-request review from all pending reviewers.

PRs should generally not be merged with pending reviews.

If the PR author decides to make any substantial changes that go beyond of what the reviewers already approved, they can re-request an already accepted review after updating the PR.

## Licenses

See the file called [LICENSE](LICENSE) for terms applying to your contribution.

### New files

When adding a new Rust file, please ensure that it starts with this precise text:

```
// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
```

When adding a new TOML file, please ensure that it starts with this precise text:

```
# This file is part of ICU4X. For terms of use, please see the file
# called LICENSE at the top level of the ICU4X source tree
# (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
```

For non-Rust/TOML files that support comments, please include a comment with the above content using the comment syntax of the applicable file format near the top of the file as practical for the file format and use case.

### Importing code

When importing pre-existing code, please observe the following:

#### Rust crates

When importing existing (Apache-2.0 OR MIT)-licensed code with pre-existing license header, please add the following above the pre-existing license header (replacing `CRATE_NAME` with the name of the original crate):

```
// The following code started as an import from CRATE_NAME which carried
// the following notice (the file locations have been superseded by the file
// called LICENSE at the top level of the ICU4X source tree):
```
_(followed by the original Apache-2.0 OR MIT license boilerplate of the Rust file)_

Also add the text from "New files" section at the very beginning of each Rust source file.

Copy incoming MIT license copyright notices from the imported code into the top-level LICENSE file.

Use the `license-file` key in `Cargo.toml` to refer to the LICENSE file.

#### Code from ICU4C/J

When porting code from ICU4C or ICU4J, indicate in source code comments that a piece of code is ported under its original license using the following comment (replacing `ICU4C` with `ICU4J` as applicable):

```
// The following code started as a port from ICU4C which carried the
// following notice:
```
_(followed the original boilerplate from the ICU4C/ICU4J source file)_

Also add the text from "New files" section at the very beginning of each Rust source file.

If you port code that is under a [third-party license](https://github.com/unicode-org/icu/blob/d95621c57f2becc1efd1be1d5c914624a715dac0/icu4c/LICENSE#L78-L414) in ICU4C/J as of the linked revision of the ICU4C LICENSE file and whose license isn't yet in the ICU4X LICENSE file, add a note about the part of code and the third-party license to the exception list in the LICENSE file, include the third-party license at the end of the LICENSE file with the title of the license, and in the code use comments to indicate that the code is under the particular third-party license.

### Tables generated from Unicode data

When tables included in Rust files are generated from Unicode data, please make the generator generate this comment above the tables:

```
// The following tables have been generated from Unicode data which carried
// the following notice:
```
_(followed by the original boilerplate from Unicode data)_

### Other cases

Please discuss first.
