Positions on Testing and Documentation
======================================

*These positions are held by sffc@ and not necesarilly the ICU4X Technical Committee.*

## Always Document and Test Assumptions

We have invariants and assumptions throughout the ICU4X code base. Often, we are able to enforce invariants via constructors that return a Result, but there will inevitably be cases where we need to pinky-promise a certain non-safety invariant and enforce it via code review.

When we introduce such invariants, we should _always_:

1. Write Documentation! Never have assumptions that are not written down in comments, and preferably in API docs.
2. Write Tests! Since action-at-a-distance could violate pinky-promise invariants, we need good test coverage for them.

I consider (1) and (2) to be requirements for shipping a stable feature. There is some flexibility, of course, in the extent of the documentation and the tests, but basic documentation and basic tests are not optional to me.

Further, the documentation and tests should enforce even temporary/non-consensus invariants. The documentation in the repository should reflect the state of the code base. If the invariant is relaxed or changed in the future, the documentation should be updated at the same time.
