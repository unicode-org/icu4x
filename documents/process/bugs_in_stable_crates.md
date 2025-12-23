ICU4X Policy on Bugs in Stable Crates
=====================================

*Status: Draft*

ICU4X has a thorough [Graduation Checklist](graduation.md) to help ensure that new components and features adhere to high standards. However, bugs and mistakes inevitably creep through, whether intentional or unintentional.

Major semver releases, like 2.0 or 3.0, give an opportunity to make large changes that fix old bugs and mistakes. This document proposes a policy for how to handle situations when a major release is far away.

This document is not comprehensive and may be revised as the team learns more.

## Principle 0: Document the Status Quo

When a bug or mistake is discovered, the first step should always be to improve documentation around the status quo: writing down behavior and invariants, even if they are suboptimal. This exercise improves communication and sets expectations.

## Principle 1: Hysteresis

Features with legitimate use cases should be retained even if they wouldn't meet the bar for adding a new feature.

Demonstrating how a feature of ICU4X could be used to solve a real-world problem is sufficient justification for retaining a feature, even if the problem is considered niche or rare.

Crucially, although this principle calls for the feature to be retained, it need not have the same shape.

This principle does not apply to call sites that the TC considers to be explicitly against i18n best practices, such as the attempted parsing of localized formatter output.

## Principle 2: Keep Client Code Working

Clients using ICU4X to solve their legitimate use cases should not need to update their code, unless doing so is unavoidable to deliver significantly improved metrics or correctness to the most common user.

Behavioral and semver edge cases can break so long as there is a well-lit migration path.

## Principle 3: Reduce Cognitive Dissonance

Avoid situations where a type is rewired to have behavior that is contrary to the name of the type. Favor designs that embrace the names of existing types.

## Principle 4: Design Forward

When adding new APIs, it is okay to be inconsistent with existing APIs if the new design is more aligned with ICU4X's mission. An issue should be opened to update the old APIs in the next major release.

The old API can be deprecated and replaced on a case-by-case basis, such as when it encourages bad practices. In general, small improvements to names of things do not justify deprecation.
