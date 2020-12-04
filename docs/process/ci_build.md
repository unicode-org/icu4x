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
    * The preferred convention by Github Actions is that action repository maintainers use `v1` and `v2` as tags that point to the commits that are tagged with the latest version within that major version. (Ex: `v1` points to `v1.2.3`, `v2` points to `v2.8.9.1`). Not all actions adhere to this guideline, and instead only have `vX.Y.Z` tags without a `vX` tag to the latest `vX.*.*`.
  * In this simplistic example, the way to allow a Pull Request to run the same checks after each new commit is pushed to the PR, in the same way that checks are run after a merge to `master`, is to change `.on.pull_request.branches` to [match all origin branch names](https://github.com/unicode-org/icu4x/commit/16ae4611738fbe94b36e17b77aee6cc541c0a171).

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
