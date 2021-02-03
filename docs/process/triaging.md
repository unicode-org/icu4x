Issue Triaging
==============

ICU4X uses GitHub for tracking feature requests and work items.  The following queries omit issues having the **discuss** label.

- All *open* issues should have a type label.
    - [Query: issues needing a type](https://github.com/unicode-org/icu4x/issues?q=is%3Aissue+is%3Aopen+-label%3AT-bug+-label%3AT-core+-label%3AT-docs+-label%3AT-enhancement+-label%3Ainvalid+-label%3Aquestion+-label%3AT-task+-label%3AT-techdebt+-label%3AT-tests+-label%3Aduplicate+-label%3Adiscuss)
- All valid issues should have a component label.
    - [Query: issues needing a component](https://github.com/unicode-org/icu4x/issues?q=is%3Aissue+-label%3AC-data+-label%3AC-datetime+-label%3AC-locale+-label%3AC-meta+-label%3AC-numbers+-label%3AC-pluralrules+-label%3AC-process+-label%3AC-test-infra+-label%3AC-unicode+-label%3Ainvalid+-label%3Aduplicate)
- All *open* issues should have an assignee or the **help wanted** label.
    - [Query: open issues needing assignee or help wanted](https://github.com/unicode-org/icu4x/issues?q=is%3Aissue+is%3Aopen+-label%3A%22help+wanted%22+-label%3Adiscuss+no%3Aassignee)
- All *closed* issues should have a resolution, linked pull request, or the **T-task**, **question**, **invalid**, or **duplicate** label.
    - [Query: closed issues needing resolution or linked PR](https://github.com/unicode-org/icu4x/issues?q=is%3Aissue+is%3Aclosed+-linked%3Apr+-label%3AR-as-designed+-label%3AR-duplicate+-label%3AR-needs-more-info+-label%3AR-obsolete+-label%3AR-out-of-scope+-label%3AR-fixed-elsewhere+-label%3Aquestion+-label%3Ainvalid+-label%3AT-task+-label%3Aduplicate)
- If an issue is open and not labeled **backlog**, the issue should be actionable. Open issues should be assigned to a milestone.
    - [Query: open issues neeting a milestone](https://github.com/unicode-org/icu4x/issues?q=is%3Aopen+is%3Aissue+no%3Amilestone+-label%3Abacklog+-label%3Adiscuss)
- If an issue is unresolved but lower-priority or not immediately actionable, it should get the **backlog** label and remain open.  The backlog should be checked periodically for issues that should be re-prioritized.
    - [Query: most recently updated backlog issues](https://github.com/unicode-org/icu4x/issues?q=is%3Aissue+label%3Abacklog+sort%3Aupdated-desc+)

## Fields

### Type

Labels starting with `T-` are *type* labels, indicating the type of deliverable for the issue.  Every issue should have one:

- **T-bug** = a defect in existing code.
- **T-core** = a high-priority improvement or new feature.
- **T-docs** = relates to documentation, including user guide, architecture design, team processes, and API docs.
- **T-enhancement** = a lower-priority improvement or new feature.
- **T-task** = a task, not a code change.
- **T-techdebt** = the issue has no user-facing effect
- **T-tests** = the issue can be addressed by unit testing.

Additional labels that can be used in place of a type:

- **T-invalid** = spam, etc.
- **T-question** = an issue that can be addressed in the discussion thread without checking in any code or documentation.

### Component

Labels starting with `C-` are *component* labels, indicating the functional component for the issue.  Every issue should have one.  Components may be added periodically; for the latest set of components, see the [label list](https://github.com/unicode-org/icu4x/labels?q=C-).

### Assignee

The assignee is the user who is *accountable* for the issue: tracking its progress, obtaining the necessary approvals, and so forth.  The assignee is often the same as the reporter.  The assignee is not necesarilly the same as the user who is *responsible* for writing the necessary code fixes.  Users interested in being *informed* or *consulted* can subscribe to the issue.

For more on the difference between *responsible*, *accountable*, *consulted*, and *informed*, see the [RACI Matrix](https://en.wikipedia.org/wiki/Responsibility_assignment_matrix).

An issue may have the **help wanted** label if there is no assignee.

### Milestone

All open issues, except for those with the **backlog** label, should be assigned to a milestone.  Milestones may be associated with a timeline (e.g., end of quarter) or release (e.g., version 0.1).  All milestones should have a due date, and milestones with approaching due dates will be viewed in the weekly meeting.

### Resolution

All *closed* issues should have either (1) the "question" type, (2) a linked pull request, or (3) one of the following labels:

- **R-duplicate** = the issue is a duplicate of some other issue.
- **R-needs-more-info** = the issue might be valid, but the subcommittee either does not understand the issue or was unable to reproduce it.  The reporter should provide more information.
- **R-obsolete** = the issue is superseded or no longer relevant.
- **R-as-designed** = the issue is valid, but the subcommittee has concluded that the library is working as intended.
- **R-fixed-elsewhere** = the issue was fixed, but there is no PR to link; for example, it was fixed in the wiki or in CLDR.

### Area

An issue may have one or more *area* labels, indicating subject areas that the issue relates to.  The list of areas may grow over time.  Area labels start with `A-`.

### Optional Labels

The following labels are optional and can be applied to an issue if appropriate:

- **backlog** = the issue is not fixed, but it could be revisited in the future.
- **good first issue** = this would be good for a new contributor.
- **v1** = revisit this issue before launching ICU4X v1 stable.
