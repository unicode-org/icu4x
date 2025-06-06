import { Decimal } from "icu4x"
import { DecimalFormatter } from "icu4x"
import { Locale } from "icu4x"
export function format(selfLocaleName, selfGroupingStrategy, valueV) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = DecimalFormatter.createWithGroupingStrategy(selfLocale,selfGroupingStrategy);
    
    let value = Decimal.fromString(valueV);
    
    let out = self.format(value);
    

    return out;
}
