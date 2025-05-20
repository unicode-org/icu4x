import { Collator } from "icu4x"
import { CollatorOptions } from "icu4x"
import { Locale } from "icu4x"
export function compare(selfLocaleName: string, selfOptionsStrength: CollatorStrength, selfOptionsAlternateHandling: CollatorAlternateHandling, selfOptionsMaxVariable: CollatorMaxVariable, selfOptionsCaseLevel: CollatorCaseLevel, left: string, right: string);
