import { Locale } from "icu4x"
import { TitlecaseMapper } from "icu4x"
import { TitlecaseOptions } from "icu4x"
export function titlecaseSegment(s, localeName, optionsLeadingAdjustment, optionsTrailingCase) {
    
    let self = new TitlecaseMapper();
    
    let locale = Locale.fromString(localeName);
    
    let options = TitlecaseOptions.fromFields({
        leadingAdjustment: optionsLeadingAdjustment,
        trailingCase: optionsTrailingCase
    });
    
    let out = self.titlecaseSegment(s,locale,options);
    

    return out;
}
