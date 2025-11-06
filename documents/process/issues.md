ICU4X Issue Reporting Guidelines
================================

The team welcomes well-researched bug reports and feature requests in the GitHub issue tracker.

Follow these guidelines when opening an issue:

1. **Search the issue tracker** to find if there is already an issue for the problem or feature request. Include closed issues in case the bug had previously been closed as Working As Intended.
    - If you find an issue, you can add a thumbs-up reaction to the original message to "vote" for the issue.
    - It is useful for the team to understand _use cases_ when evaluating bugs and feature requests. Please post your use cases in a reply to the issue you found.
    - Please refrain from posting comments such as "+1" or "Is there an update on this?" unless the last comment on the issue is at least 6 months ago.
2. **Post clarifying questions** in [the "Discussions" tab](https://github.com/unicode-org/icu4x/discussions).
    - The issue tracker should be for _bugs_ and _feature requests_. If you are having trouble following a tutorial or have some other question about _using_ ICU4X, please post your question in the "Discussions" tab.
3. **Verify whether this is a data issue.** ICU4X is a data-driven library and many bugs observed in ICU4X may be issues in CLDR instead.
    - Read the [CLDR issue reporting guidelines](https://github.com/unicode-org/cldr/blob/main/documents/requesting_changes.md) for more information on the CLDR process.
4. **Check whether the issue has been fixed.** Sometimes if you pull ICU4X from the main branch, the issue is not reproducible.
    - To do this easily in your Cargo.toml file, replace your ICU4X dependency line with `icu = { git = "https://github.com/unicode-org/icu4x.git" }`
5. **Isolate the problem.** Small, reproducible issues are more actionable than large, vague ones.
    - If reporting a bug, follow this template: "I did X, and I expected Y, but I observed Z."
    - Provide as much detail as possible to reproduce the state where you observed the unexpected behavior.
6. **Consider attaching code in a pull request.** If you open a pull request with a failing test case, it makes it easier to reproduce the problem.
    - Of course, you are also welcome to fix the issue in the pull request.
    - For more information on contributing pull requests, see [CONTRIBUTING.md](https://github.com/unicode-org/icu4x/blob/main/CONTRIBUTING.md).

Once you are ready, [file your new issue](https://github.com/unicode-org/icu4x/issues/new/choose).

If you follow these steps, you are more likely to receive a timely and productive response from the development team.
