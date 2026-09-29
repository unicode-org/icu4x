// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO(agent): add tests for things like:
// - skeleton <=> fieldbag:
//     - all bags (which are enumerable) should produce skeletons
//     - most such skeletons should round-trip to the same bag
// - fieldset builder <=> fieldbag:
//     - all bags (which are enumerable) should produce builders
//     - all such builders should produce a valid composite fieldset
//     - all builders (which are enumerable) should produce bags
//     - round-tripping here is complicated; write a clever test for it
// - fieldset (formatter) <=> fieldset builder
//     - maybe this belongs in a different test file?
//     - all fieldsets (which are enumerable) should produce builders
//     - all such builders should round-trip to the same fieldset
