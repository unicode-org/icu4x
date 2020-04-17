Issue Triaging
==============

OmnICU uses GitHub for tracking feature requests and work items.

- All issues should have a type label.
    - [Query: issues needing a type](https://github.com/unicode-org/omnicu/issues?q=is%3Aissue+-label%3Abug+-label%3A%22core+feature%22+-label%3Adocumentation+-label%3Aenhancement+-label%3Aquestion+-label%3Ainvalid+)
- All *open* or *backlog* issues should have and assignee or **help wanted** label.
    - [Query: open issues needing assignee or help wanted](https://github.com/unicode-org/omnicu/issues?q=is%3Aissue+is%3Aopen+-label%3A%22help+wanted%22+no%3Aassignee+)
    - [Query: backlog issues needing assignee or help wanted](https://github.com/unicode-org/omnicu/issues?q=is%3Aissue+is%3Aclosed+label%3Abacklog+-label%3A%22help+wanted%22+no%3Aassignee)
- All *closed* issues should have a resolution, linked pull request, or the **question** type.
    - [Query: closed issues needing resolution or linked PR](https://github.com/unicode-org/omnicu/issues?q=is%3Aissue+is%3Aclosed+-linked%3Apr+-label%3Aquestion+-label%3Abacklog)
- If an issue is open, the issue should be actionable. Open issues should generally have an activity update once every 60 days.
    - [Query: least recently updated open issues](https://github.com/unicode-org/omnicu/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-asc)
- If an issue is unresolved but lower-priority or not immediately actionable, it should get the **backlog** label and be closed.  The backlog should be checked periodically for issues that should be reopened.
    - [Query: most recently updated backlog issues](https://github.com/unicode-org/omnicu/issues?q=is%3Aissue+is%3Aclosed+label%3Abacklog+sort%3Aupdated-desc+)

## Fields

### Type

The following labels are types; every issue should have one:

- **bug** = a defect in existing code.
- **core feature** = a high-priority improvement or new feature.
- **documentation** = relates to documentation, including user guide, architecture design, team processes, and API docs.
- **enhancement** = a lower-priority improvement or new feature.
- **question** = an issue that can be addressed in the discussion thread without checking in any code or documentation.
- **invalid** = spam, etc.

### Assignee

The assignee is the user who is the "champion" for the issue: tracking its progress, obtaining the necessary approvals, and so forth.  The assignee is often the same as the reporter.  The assignee is not necesarilly the same as the user who is writing the necessary code fixes.

An issue may have the **help wanted** label if there is no assignee.

### Resolution

All *closed* issues should have either (1) the "question" type, (2) a linked pull request, or (3) one of the following labels:

- **backlog** = the issue is not fixed, but it could be revisited in the future.
- **duplicate** = the issue is a duplicate of some other issue.
- **needs more info** = the issue might be valid, but the subcommittee either does not understand the issue or was unable to reproduce it.  The reporter should provide more information.
- **obsolete** = the issue is superseded or no longer relevant.
- **wontfix** = the issue is valid, but the subcommittee has concluded that the library is working as intended.

### Optional Labels

The following labels are optional and can be applied to an issue if appropriate:

- **good first issue** = this would be good for a new contributor.
