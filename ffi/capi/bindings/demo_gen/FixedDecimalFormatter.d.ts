import { FixedDecimalFormatter } from "icu4x"
import { Locale } from "icu4x"
import { SignedFixedDecimal } from "icu4x"
export function format(name: string, groupingStrategy: FixedDecimalGroupingStrategy, f: number, magnitude: number);
export function numberingSystem(name: string, groupingStrategy: FixedDecimalGroupingStrategy);
