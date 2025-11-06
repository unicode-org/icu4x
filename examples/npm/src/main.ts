// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

import { Decimal, DecimalFormatter, DecimalGroupingStrategy, Locale } from 'icu';

var locale = Locale.fromString('bn');
var groupingStrategy = DecimalGroupingStrategy.Auto;
var decimal = Decimal.fromString('123');

var formatter = DecimalFormatter.createWithGroupingStrategy(
    locale,
    groupingStrategy,
);

console.log(formatter.format(decimal));
