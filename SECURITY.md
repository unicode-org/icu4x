# Security Policy

## Reporting a Vulnerability

If you have discovered a security vulnerability in ICU4X crates or any utility crates maintained in this repository, please report it privately. **Do not disclose it as a public issue.**
This gives us time to work with you to fix the issue before public exposure, reducing the chance that the exploit will be used before a patch is released.

Please submit the bug report using the [security vulnerability report form](https://github.com/unicode-org/icu4x/security/advisories/new) on GitHub.

Please provide the following information in your report:

 * A description of the vulnerability and its impact
 * How to reproduce the issue
 * The versions you know are impacted (this list need not be exhaustive)

We ask that you give us 90 days to work on a fix before public exposure.


## Supported Versions

We are happy to receive security reports for any published versions of ICU4X, including the code available on `main`.

We strive to release patches for crates for the most recent ICU4X minor release.

Depending on the severity and age of the issue, we may or may not choose to release patches of older crates. We are likely to fix security issues in at least the previous ICU4X minor release as well.
