import { Collator } from "icu4x"
import { CollatorOptions } from "icu4x"
import { Locale } from "icu4x"
export function compare(selfLocaleName, selfOptionsStrength, selfOptionsAlternateHandling, selfOptionsMaxVariable, selfOptionsCaseLevel, left, right) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let selfOptions = CollatorOptions.fromFields({
        strength: selfOptionsStrength,
        alternateHandling: selfOptionsAlternateHandling,
        maxVariable: selfOptionsMaxVariable,
        caseLevel: selfOptionsCaseLevel
    });
    
    let self = Collator.create(selfLocale,selfOptions);
    
    let out = self.compare(left,right);
    
    out = out == 0 ? '==' : out == 1 ? '>' : '<';;
    

    return out;
}
