import { Decimal, DecimalFormatter, DecimalGroupingStrategy, Locale } from "icu4x";

var locale = Locale.fromString("bn");
var groupingStrategy = DecimalGroupingStrategy.Auto;
var decimal = Decimal.fromString("123");

var formatter = DecimalFormatter.createWithGroupingStrategy(
    locale,
    groupingStrategy,
);

console.log(formatter.format(decimal));
