import { CaseMapper } from "icu4x"
import { Locale } from "icu4x"
import { TitlecaseOptions } from "icu4x"
export function lowercase(s, localeName) {
    
    let self = new CaseMapper();
    
    let locale = Locale.fromString(localeName);
    
    let out = self.lowercase(s,locale);
    

    return out;
}
export function uppercase(s, localeName) {
    
    let self = new CaseMapper();
    
    let locale = Locale.fromString(localeName);
    
    let out = self.uppercase(s,locale);
    

    return out;
}
export function titlecaseSegmentWithOnlyCaseData(s, localeName, optionsLeadingAdjustment, optionsTrailingCase) {
    
    let self = new CaseMapper();
    
    let locale = Locale.fromString(localeName);
    
    let options = TitlecaseOptions.fromFields({
        leadingAdjustment: optionsLeadingAdjustment,
        trailingCase: optionsTrailingCase
    });
    
    let out = self.titlecaseSegmentWithOnlyCaseData(s,locale,options);
    

    return out;
}
export function fold(s) {
    
    let self = new CaseMapper();
    
    let out = self.fold(s);
    

    return out;
}
export function foldTurkic(s) {
    
    let self = new CaseMapper();
    
    let out = self.foldTurkic(s);
    

    return out;
}
export function simpleLowercase(ch) {
    
    let self = new CaseMapper();
    
    let out = self.simpleLowercase(ch);
    
    out = String.fromCharCode(out);;
    

    return out;
}
export function simpleUppercase(ch) {
    
    let self = new CaseMapper();
    
    let out = self.simpleUppercase(ch);
    
    out = String.fromCharCode(out);;
    

    return out;
}
export function simpleTitlecase(ch) {
    
    let self = new CaseMapper();
    
    let out = self.simpleTitlecase(ch);
    
    out = String.fromCharCode(out);;
    

    return out;
}
export function simpleFold(ch) {
    
    let self = new CaseMapper();
    
    let out = self.simpleFold(ch);
    
    out = String.fromCharCode(out);;
    

    return out;
}
export function simpleFoldTurkic(ch) {
    
    let self = new CaseMapper();
    
    let out = self.simpleFoldTurkic(ch);
    
    out = String.fromCharCode(out);;
    

    return out;
}
