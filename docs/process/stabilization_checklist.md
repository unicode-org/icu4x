# New Feature Stabilization Checklist

Follow this checklist when promoting a new component or feature on an existing component from experimental to stable.

1. [ ] **Style guide is followed.** To enforce the style guide, see [boilerplate.md](boilerplate.md).
2. [ ] **FFI is complete.** There should be no unjustified suppressions nor entries in missing_apis.txt.
3. [ ] **Docs are complete.** In addition to covering every exported function, there needs to be a crate-level overview example, and all options should be covered by at least one docs test.
4. [ ] **Data story is complete.** The code and data should be highly modular, such that users do not need to carry more code or data than they need. The data should be zero-copy of course, and it should make use of the abstractions available in the zerovec crate. If datagen requires a new feature in order for the new data to be modularized, that feature should be implemented.
5. [ ] **Feature exhibits i18n correctness.** There should be no known gaps in localization quality, meaning that for the component in question, a user in any CLDR locale should receive an experience on par with that of a user in English. In other words, the component does not need to be feature-complete, but of the features that are supported, they should be fully implemented.
6. [ ] **API sign-off.** An i18n expert who is not on the core ICU4X team should give a stamp of approval that the given ICU4X API is future-proof and encourages best practices.
