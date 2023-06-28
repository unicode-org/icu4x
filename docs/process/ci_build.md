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
  * In the UI, for each workflow run instance, you can see each job of the workflow, and each step within each job.  For a job, you can see the console output (stdout, stderr) for each step with linkable line numbers and a search bar for searching the logs.

* This example from an [old, experimental, preliminary ICU4X workflow](https://github.com/unicode-org/icu4x/commit/466eaa3a72635b01cfc1be33e1c899e33147301a) shows a really small task executed in a naive way:
  * The `on` key config shows that the workflow is triggered only when a commit is made on `main`. Since the repository only allows commits to `main` through the Pull Request process, this workflow is unhelpful for Pull Request authors and reviewers because pull requests originating from any branch not called `main` (all of them) will not trigger the workflow.
  * There is one job, which is to build and run unit tests.
  * The first step of any job is usually to checkout the latest code in the repository. Otherwise, the fresh VM runner has an empty filesystem.
  * The first step of this job invokes `actions/checkout@v2`. These action coordinates indicate: invoke the Github Action kept in the repository https://www.github.com/actions/checkout, and use the state of that repository that the `v2` ref points to. Since `v2` is a git "ref", that means could be a git tag, a git branch name, or a git commit hash value.
    * The preferred convention by Github Actions is that action repository maintainers use `v1` and `v2` as tags that point to the commits that are tagged with the latest version within that major version. (Ex: `v1` points to `v1.2.3`, `v2` points to `v2.8.9.1`). However, be aware that not all actions adhere to this guideline, and instead only have `vX.Y.Z` tags without a `vX` tag to the latest `vX.*.*`. As an example, this problem was [observed and worked around in the `unicode-org/cldr` repo](https://github.com/unicode-org/cldr/pull/813).
  * For this simplistic example, the way to improve it in order to allow a Pull Request to run the same checks after each new commit is pushed, in the same way that checks are run on merges to `main`, is to change `on.pull_request.branches` to [match all origin branch names](https://github.com/unicode-org/icu4x/commit/16ae4611738fbe94b36e17b77aee6cc541c0a171).

## Testing a Workflow in unicode-org/icu4x

One nice aspect of Github Actions' integration in Github is that if there is a workflow that is triggered to run on pull requests, and if a pull request includes some modification to that workflow, then the pull request will be run _using the pending new changes to the workflow_.  For example, if an existing workflow (ex: `.github/workflows/build-test.yml`) is configured to run 5 benchmarks, and a pull request is made to add a 6th benchmark to that workflow, then the pull request will run 6 benchmarks, not 5.

This means that most changes to a workflow can be tested in the PR that introduces the changes. The effects are visible to the PR author and reviewers alike.

## Testing a Workflow in Your Forked Repo

There might be reasons why you want or need to test changes to Github Actions on your personal fork. Less frequently, there might be specific changes that cannot be tested via a PR because they only happen on merges to the main branch `main`. For example, API docs changes and benchmark dashboard changes should only occur on merges to main. In this case, you can use your personal fork of the upstream repo as a testing ground. The naive approach is not recommended -- to directly modify your `main` branch -- because it requires awkwardly changing your git repo during and after testing in ways that are often confusing and opposite to "git-flow" habits. 

So here are steps to test Github Actions change on your personal fork, with links to an example:

1. Create a new testing-only branch that tacks on [an extra commit to make testing-appropriate changes](https://github.com/echeran/icu4x/pull/22/commits/538176500f54594cffa1844c0244c7135ea24b84).
2. Push the testing branch to the personal fork of `icu4x` and create a [testing PR targeting personal fork's main](https://github.com/echeran/icu4x/pull/22).
3. [GitHub Actions will run a new job]((https://github.com/echeran/icu4x/actions/runs/539462550)) because the testing PR satisfies the execution trigger conditions.
4. Optional extra step - merge the testing PR to get Github Actions [execute the job that copies GH pages over to the fork of the docs repo](https://github.com/echeran/icu4x/actions/runs/539466271).

Also, part of your testing-appropriate changes can be as primitive-yet-sufficient as adding lots of stdout debugging that gets captured in the logs ([example](https://github.com/echeran/icu4x/pull/13/commits/d6168715ac644b785c920b639cd5238e7a52f806)).

## Slightly Advanced Features Used in unicode-org repos

* A "job matrix" allows you to run a collection of parameterized jobs, where you indicate which parts of the job configuration are the parameters (variables) and what values they are allowed to take. If the parameter is the OS of the VM, then you can run the same job on Linux, macOS, and Windows with little extra work
  * The parameters are defined as fields under the `strategy.matrix.<param>` key within the job, and the range of allowed values are stored as arrays. The parameters are used (string interpolated) with `${{ matrix.<param> }}` syntax. [Example](https://github.com/unicode-org/icu4x/blob/main/.github/workflows/build-test.yml#L17-L21):
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
  Here, `os` is a parameter defined under `strategy.matrix.os` for the `test` job's job matrix. `os` takes on all values in the range defined by the array `[ ubuntu-latest, macos-latest, windows-latest ]`. Every time the `test` job is run, it is run 3 times, once per possible value.
  * A job matrix can help decrease wall clock time for multiple independent long-running steps, like benchmarks. [Example](https://github.com/unicode-org/icu4x/blob/main/.github/workflows/build-test.yml#L115-L127):
  ```yml
  jobs:
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
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v2
        - name: Run benchmark
          run: |
            pushd $PWD && cd ${{ matrix.component }};
            cargo bench -- --output-format bencher | tee $OUTPUT_PATH/output.txt;
            popd
        - ...
  ```
  Here, `component` is a parameter defined under `strategy.matrix.component` for the `benchmark` job's job matrix. `component` takes on the values defined in the [YAML array](https://stackoverflow.com/a/33136212) `[ components/locid, components/uniset, components/plurals, components/datetime, utils/fixed_decimal]`
* Conditional execution of steps and jobs - you can use the `if` key to control more granularly whether a step or job can run.
  * In this [example](https://github.com/unicode-org/icu4x/blob/main/.github/workflows/build-test.yml#L168), we want the workflow to trigger on all Pull Requests and successful merges to `main`. However, when we look more granularly at the jobs within the workflow, some jobs, like regenerating API docs or benchmark dashboards, make no sense on in-flight PRs and therefore should only execute when they're fully finished, reviewed, and merged to `main`. We add the `if` key on the jobs to control the conditional execution in isolated instances that is more granular than the workflow-level triggers defined in the `on` key.
* "Uploading / downloading artifacts" is a mechanism that Github Actions provides to allow a persistence of files from one job to another within a single workflow. This can be useful since each job VM runner is created fresh, and inherits no previous state.
  * [Example upload](https://github.com/unicode-org/icu4x/blob/main/.github/workflows/build-test.yml#L213-L218):
  ```
      - name: Upload updated benchmark data (merge to main only)
        if: github.event_name == 'push' && github.ref == 'refs/heads/main'
        uses: actions/upload-artifact@v2
        with:
          path: ./benchmarks/perf/**  # use wildcard pattern to preserve dir structure of uploaded files
          name: benchmark-perf
  ```
  * [Example download](https://github.com/unicode-org/icu4x/blob/main/.github/workflows/build-test.yml#L246-L250):
  ```
    - name: Download previous content destined for GH pages
      uses: actions/download-artifact@v2
      with:
        path: ./copy-to-ext-repo/benchmarks/perf
        name: benchmark-perf
  ```
  * There is no mechanism to persist data storage across workflow instantiations. The only way to persist / store data across workflow instances is through making commits on a branch within the git repo itself. A benchmark dashboard requires the accumulation of data points computed from each invocation. Therefore, the benchmark action that we use relies on creating git commits on a branch to store this historical information over time.


## Examples of Various CI Functionality

_Note: the `*.yml` workflow files should should provide detailed documentation for the individual steps. These are only broad examples of what the GitHub Actions can do._

### Publish artifacts to GitHub pages

API docs, the WASM demo, and benchmarking graphs are uploaded to a Google Cloud Storage bucket (accessible by Git ref), and published to this repository's
[GitHub Pages](https://unicode-org.github.io/icu4x) after each commit to `main`.

### Benchmarking

We use the [rhysd/github-action-benchmark](https://github.com/rhysd/github-action-benchmark). It does the following:
  1. For a handful of programming languages (including Rust), it runs the most common benchmark tool
  1. It converts the benchmark tool's output into a JSON file
  1. We upload the JSON file and the generated HTML dashboard to our GCS bucket
  1. The graphs then get published on GitHub Pages

### Run CI checks locally

Developers [need an easy way to run the same commands and options locally as they are run in CI](https://github.com/unicode-org/icu4x/issues/334).
Currently, there is no official way to invoke a Github Actions workflow locally.
Since we run tests on all major OS types, we need a cross-platform solution to running commands.
Rust's `cargo-makefile` is a plugin for the `cargo` build tool that allows creation of a sequence of commands and options using an alias.
Using `cargo-makefile`, the CI commands were rewritten to use the new aliases.
`cargo-makefile` has built-in support for the inline inclusion of scripts written in [Duckscript](https://github.com/sagiegurari/duckscript)
[Here is an example of an inline script in a `cargo-makefile` task](https://github.com/unicode-org/icu4x/pull/381/files#diff-9375fd04332c86472d7be397ef09428cb86babd8826880a5835bd1d1c1bdbc08R23-R48) that ensures that all new source files have a specific license header.

### Code coverage

Code coverage is configured in a separate workflow since it is completely independent of the operations for build, test, format/style-checking, linting/error-prone-checking, API docs, benchmarking.
Two code coverage dashboard services exist:
  * [Codecov](https://codecov.io/)
    - [config for Codecov](https://github.com/unicode-org/icu4x/commit/fd524a91df3f5aef2efb908adddd051827972328#diff-a2115d277b5ca5a2f09a999e53440839cf332b94da177f3d1766334555b0f7c6R49-R53)
  * [Coveralls](https://coveralls.io/)
    - [config for Coveralls](https://github.com/unicode-org/icu4x/commit/00c514f3a189787c4e77704eb98714e6b6430d37#diff-a2115d277b5ca5a2f09a999e53440839cf332b94da177f3d1766334555b0f7c6R36-R40)
Both code coverage dashboard services automatically support most output file formats for most programming languages' code coverage tools.
The Github Actions for both code coverage services are configured to automatically add a PR comment with the code coverage report for the PR's changes. The actions are smart enough to invalidate/close old comments and only leave the latest open comment for the latest state of code in the PR.

### Caching

Each job within a workflow is run in a new, clean runner VM instance. However, within a single workflow, if multiple jobs depend on the state after a single compilation process, when that compilation takes a long time, then the jobs will incur redundant effort that adds to the total time. In cases where the compilation generates libraries / packages / artifacts that can be re-used in subsequent steps, these output artifacts can be cached. The cache lasts indefinitely, meaning that the artifacts are reused from job-to-job and workflow-instance-to-workflow-instance.

The [Github Actions caching](https://docs.github.com/en/actions/guides/caching-dependencies-to-speed-up-workflows) enables caching after specifying the basics: 1) what is to be cached (the path of files in the filesystem), 2) the specific key to register in the cache for those files. Specifying what is to be cache is build tool dependent, ex: `.m2/**` for Java Maven builds, or `target` for Rust Cargo builds. How you specify the key determines the uniqueness of the build and its dependencies.

Previously in ICU4X, [caching was used for the Rust build](https://github.com/unicode-org/icu4x/blob/d6734764840095914f2c9be32a7f27eb544688cd/.github/workflows/build-test.yml#L32-L54) in the parts of the cargo compiler's cache of repositories, index of known crates, and the compiled crate binaries. Differences in the build tool's main config file (`Cargo.toml`) indicate a potential change in dependencies, so the file digest hash of that file is used in constructing the cache key associated with the dependency artifacts location. The version of Rust matters for compatibility between some dependencies, so it probably needs to be included in cache key string.  (By contrast, for Java, the inclusion of the language version number in the cache key might be less important since Java byte code is generally forwards compatible with future versions of the language.)

Within a job, the cache step (the job step using the cache action `actions/cache`) should be specified early. At the point in the job where it is declared, it will restore any files to the path specified if the key specified exists. Use of the cache action will create an extra step at the end of the job, if the rest of the job successfully completes. This extra last step will construct the cache key string, and only if the key is not present, will it store the files in the cache. Otherwise, the last step is a no-op.

Troubleshooting cache misconfiguration can be tricky, but the most likely error is that the cache key is not specific enough. When that happens, there can be an expectation that a new, different set of files will replace the old set of files under that key. However, unlike a map, there is a "write once" behavior, not an "overwrite" behavior, so there is a no-op even when the path contains a different set of files than usual. On the opposite end, if the key is too specific, then you risk creating duplicate copies of the same files, which can prevent the proper re-use of files that caching is meant to solve, if it doesn't already incur excess usage above the [cache storage capacity](https://docs.github.com/en/actions/guides/caching-dependencies-to-speed-up-workflows#usage-limits-and-eviction-policy) first. If you need to change the semantic of a cache key, you can either: 1) append a "-v2" to the end in order to create a new, _unused, unique_ string, or 2) stop using the key for 7 days, after which [the key is evicted from the cache](https://docs.github.com/en/actions/guides/caching-dependencies-to-speed-up-workflows#usage-limits-and-eviction-policy).
