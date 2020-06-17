# Create Ownership Rules

*NOTE*: Rules came from discussion on issue #72
***

Each crate released on crates.io should have:

1. Two or more owners
2. Owners should be from different teams, functional areas or companies
3. Team ownership is required for future maintenance

## Two Owners

More than one person on ICU4X should be familiar with and supportive of a public crate maintained by the team.

***
*NOTE*: It's owner's responsibility to find replacement if they are leaving the project/crate.
***

## Owners from Different Areas

To promote healthy discussion, and ensure we work on impactful crates that cover needs of wide audience vs. one of the members we should have support for the crate from at least two vendors/teams/companies.

## Team Ownership

To ensure further maintenance and publishing rights, team ownership should be added to each public crate.

```
cargo owner -a github:unicode-org/teamname
```
