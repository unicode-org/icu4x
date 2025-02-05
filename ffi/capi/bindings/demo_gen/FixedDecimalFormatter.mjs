import { FixedDecimalFormatter } from "icu4x"
import { Locale } from "icu4x"
import { SignedFixedDecimal } from "icu4x"
export function format(fixedDecimalFormatterLocaleName, fixedDecimalFormatterGroupingStrategy, valueF, valueMagnitude) {
    
    let fixedDecimalFormatterLocale = Locale.fromString(fixedDecimalFormatterLocaleName);
    
    let fixedDecimalFormatter = FixedDecimalFormatter.createWithGroupingStrategy(fixedDecimalFormatterLocale,fixedDecimalFormatterGroupingStrategy);
    
    let value = SignedFixedDecimal.fromNumberWithLowerMagnitude(valueF,valueMagnitude);
    
    let out = fixedDecimalFormatter.format(value);
    

    return out;
}
