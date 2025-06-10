import { Decimal } from "icu4x"
import { Locale } from "icu4x"
import { PluralOperands } from "icu4x"
import { PluralRules } from "icu4x"
export function categoryFor(selfLocaleName, opXV) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = PluralRules.createCardinal(selfLocale);
    
    let opX = Decimal.fromString(opXV);
    
    let op = PluralOperands.fromFixedDecimal(opX);
    
    let out = self.categoryFor(op);
    
    out = out?.value || 'None';;
    

    return out;
}
