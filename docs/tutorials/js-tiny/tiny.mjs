// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// ICU4X uses fetch when available, but fetch in Node.js is broken,
// so delete the function to force the wasm to be loaded via the fs module.
// See <https://github.com/rust-diplomat/diplomat/issues/283>.
delete globalThis.fetch;

import {ICU4XLocale, ICU4XDataProvider, ICU4XFixedDecimalFormatter, ICU4XFixedDecimal } from './lib/index.mjs';

const locale = ICU4XLocale.create_from_string("bn");
const provider = ICU4XDataProvider.create_compiled();

const format = ICU4XFixedDecimalFormatter.create_with_grouping_strategy(provider, locale, "Auto");

const decimal = ICU4XFixedDecimal.create_from_i32(1000007);
decimal.multiply_pow10(-2);

const result = format.format(decimal);
console.log("Output is", result);
