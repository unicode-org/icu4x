// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// ICU4X uses fetch when available, but fetch in Node.js is broken,
// so delete the function to force the wasm to be loaded via the fs module.
// See <https://github.com/rust-diplomat/diplomat/issues/283>.
delete globalThis.fetch;

import {Locale, DecimalFormatter, Decimal, DecimalGroupingStrategy } from './lib/index.mjs';

const locale = Locale.fromString("bn");

const format = DecimalFormatter.createWithGroupingStrategy(locale, DecimalGroupingStrategy.Auto);

const decimal = Decimal.fromNumber(1000007);
decimal.multiplyPow10(-2);

const result = format.format(decimal);
console.log("Output is", result);
