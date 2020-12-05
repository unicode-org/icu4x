# Continuous Integration and Builds

## Overview

Currently, we use Github Actions to run our continuous integration & builds.

Basic concepts:

* A step is the smallest unit of work in Github Actions. It can be as simple as running a shell command.
* A step is versatile because it can also invoke external [community-contributed functions](https://github.com/marketplace?type=actions) ("actions"). [Actions](https://docs.github.com/en/free-pro-team@latest/actions/learn-github-actions/introduction-to-github-actions) can be used for simple operations like checking out the latest code in the repository, or slightly more complicated tasks like running a benchmarking tool and formatting the results. Actions are usually kept small in scope. They are implemented as repositories on Github whose contents implement interfaces defined by Github Actions.
* A job runs multiple steps **serially**.
* A job groups together related steps that achieve a larger task (ex: run unit tests)
* Each job instance is run in a completely new VM instance ("runner").
* A workflow runs multiple jobs **in parallel**.
* Workflows are run by Github Actions any time certain trigger events occur. Each workflow configures what trigger events conditions must be met for it to to be invoked (ex: only for Pull Requests; only on merges to the main branch)
* Jobs within a workflow can be made to run in a particular order by specifying job dependencies. This creates the directed acyclic graph (DAG) relationship of job execution.

## Getting Started

* This [intro page](https://docs.github.com/en/free-pro-team@latest/actions/learn-github-actions/introduction-to-github-actions) shows the following
  * A Github Action workflow is written in YAML syntax, and must be placed in `.github/workflows`.
  * A separate tab within the Github UI for a repository shows the history of all Actions runs for the repository.
  * In the UI, for each workflow run instance, you can see each job of the workflow, and each step within each job.  For a job,you can see the console output (stdout, stderr) for each step with linkable line numbers and a search bar for searching the logs.

* This example from an [old, experimental, preliminary ICU4X workflow](https://github.com/unicode-org/icu4x/commit/466eaa3a72635b01cfc1be33e1c899e33147301a) shows a really small task executed in a naive way:
  * The `on` key config shows that the workflow is triggered only when a commit is made on `master`. Since the repository only allows commits to `master` through the Pull Request process, this workflow is unhelpful for Pull Request authors and reviewers because pull requests originating from any branch not called `master` (all of them) will not trigger the workflow.
  * There is one job, which is to build and run unit tests
  * The first step of any job is usually to checkout the latest code in the repository. Otherwise, the fresh VM runner has an empty filesystem.
  * The first step of this job invokes `actions/checkout@v2`. This means to invoke the Github Action kept in the repository https://www.github.com/actions/checkout, and use the state of that repository that the `v2` ref points to. Since `v2` is a git "ref", that means could be a git tag, a git branch name, or a git commit hash value.
    * The preferred convention by Github Actions is that action repository maintainers use `v1` and `v2` as tags that point to the commits that are tagged with the latest version within that major version. (Ex: `v1` points to `v1.2.3`, `v2` points to `v2.8.9.1`). However, be aware that not all actions adhere to this guideline, and instead only have `vX.Y.Z` tags without a `vX` tag to the latest `vX.*.*`. As an example, this problem was [observed and worked around in the `unicode-org/cldr` repo](https://github.com/unicode-org/cldr/pull/813).
  * For this simplistic example, the way to improve it to allow a Pull Request to run the same checks after each new commit is pushed, in the same way that checks are run on merges to `master`, is to change `on.pull_request.branches` to [match all origin branch names](https://github.com/unicode-org/icu4x/commit/16ae4611738fbe94b36e17b77aee6cc541c0a171).

## Testing a Workflow

One nice aspect of Github Actions' integration in Github is that if there is a workflow that is triggered to run on pull requests, and if a pull request includes some modification to that workflow, then the pull request will be run _using the pending new changes to the workflow_.  For example, if an existing workflow (ex: `.github/workflows/build-test.yml`) is configured to run 5 benchmarks, and a pull request is made to add a 6th benchmark to that workflow, then the pull request will run 6 benchmarks, not 5.

This means that most changes to a workflow can be tested in the PR that introduces the changes. The effects are visible to the PR author and reviewers alike.

Less frequently, there might be specific changes that cannot be tested via a PR because they only happen on merges to the main branch `master`. For example, API docs changes and benchmark dashboard changes should only occur on merges to master. In this case, you can use your personal fork of the upstream repo as a testing ground. You can remove branch restrictions from pushing directly to `master` on your fork repo's settings, and you can push the changes you want to test directly on `master`. An alternative strategy for using your fork to do testing is, instead of making feature changes directly on `master`, you can maintain the changes on a feature branch as normal for a "git-flow" workflow. And you can create a special testing branch from the feature branch that maintains an extra commit at its head that loosens restrictions and makes fork-specific configuration changes. For example, when testing fixes to dashboards in `unicode-org/icu4x`, a feature branch `ci-bench-per-component` was created. Then, a testing branch [`ci-bench-per-component-test`](https://github.com/echeran/icu4x/pull/13/commits) was created. Feature changes were made on the feature branch, and the `*-test` branch was always rebased on top of the feature branch in order to maintain fork/testing reconfigurations, such as [allowing `master`-only jobs to run on all branches](https://github.com/echeran/icu4x/pull/13/commits/5e83cb4f97a6629b9bfa330aaeb73b3c523f0a8f), making the [API docs / benchmarks copy destination to be on the personal fork](https://github.com/echeran/icu4x/pull/13/commits/61953d9275d3853d96f43ed95e2ff0e9dc5fdd5f), and adding lots of stdout debugging that gets captured in the logs ([example](https://github.com/echeran/icu4x/pull/13/commits/d6168715ac644b785c920b639cd5238e7a52f806)). The `*-test` branch (`ci-bench-per-component-test`) was pushed to the personal fork after each change to trigger workflow test invocations. The feature branch was largely kept locally until the feature work was done, but the important point was to preserve the invariant that the feature branch (`ci-bench-per-component`) was always in a good state to be turned into a PR on the upstream repo.

## Slightly Advanced Features Used in unicode-org repos

* A "job matrix" allows you to run a collection of parameterized jobs, where you indicate which parts of the job configuration are the parameters (variables) and what values they are allowed to take. If the parameter is the OS of the VM, then you can run the same job on Linux, macOS, and Windows with little extra work
  * The parameters are defined as fields under the `strategy.matrix` key within the job, and the range of allowed values are stored as arrays. The parameters are used (string interpolated) with `${{ matrix.<param> }}` syntax. [Example](https://github.com/unicode-org/icu4x/blob/master/.github/workflows/build-test.yml#L17-L21):
```yml
jobs:
  test:
    strategy:
      fail-fast: false
      matrix:
        os: [ ubuntu-latest, macos-latest, windows-latest ]
    runs-on: ${{ matrix.os }}
    steps:
    - uses: actions/checkout@v2
    - ...
```
  * A job matrix can help decrease wall clock time for multiple independent long-running steps, like benchmarks. [Example](https://github.com/unicode-org/icu4x/blob/master/.github/workflows/build-test.yml#L115-L127):
```
  benchmark:
    strategy:
      fail-fast: false
      matrix:
        component:
          - components/locid
          - components/uniset
          - components/plurals
          - components/datetime
          - utils/fixed_decimal
```
* Conditional execution of steps and jobs - you can use the `if` key to control more granularly whether a step or job can run.
  * In this [example](https://github.com/unicode-org/icu4x/blob/master/.github/workflows/build-test.yml#L168), we want the workflow to trigger on all Pull Requests and successful merges to `master`, but some steps, like regenerating API docs or benchmark dashboards, make no sense on in-flight PRs and therefore should only execute on merges to `master`.
* "Uploading / downloading artifacts" is a mechanism that Github Actions provides to allow a persistence of files from one job to another within a single workflow. This can be useful since each job VM runner is created fresh, and inherits no previous state.
  * [Example upload](https://github.com/unicode-org/icu4x/blob/master/.github/workflows/build-test.yml#L213-L218):
```
      - name: Upload updated benchmark data (merge to master only)
        if: github.event_name == 'push' && github.ref == 'refs/heads/master'
        uses: actions/upload-artifact@v2
        with:
          path: ./dev/**  # use wildcard pattern to preserve dir structure of uploaded files
          name: benchmark-perf
```
  * [Example download](https://github.com/unicode-org/icu4x/blob/master/.github/workflows/build-test.yml#L246-L250):
```
    - name: Download previous content destined for GH pages
      uses: actions/download-artifact@v2
      with:
        path: ./copy-to-ext-repo/dev
        name: benchmark-perf
```
## Implementation of Various CI Aspects

TODO: elaborate on each

* Copy API docs to separate repo
* Benchmarking - create data, store in separate branch, copy to same repo as API docs
* Run CI checks locally - Rust's `cargo-makefile` to allow a local way to run a sequence of commands, rewire CI test to use commands, runs cross-platform
* Code coverage
