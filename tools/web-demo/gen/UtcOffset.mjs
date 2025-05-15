import { UtcOffset } from "icu4x"
export function seconds(selfOffset) {
    
    let self = UtcOffset.fromString(selfOffset);
    
    let out = self.seconds;
    

    return out;
}
export function isNonNegative(selfOffset) {
    
    let self = UtcOffset.fromString(selfOffset);
    
    let out = self.isNonNegative;
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function isZero(selfOffset) {
    
    let self = UtcOffset.fromString(selfOffset);
    
    let out = self.isZero;
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function hoursPart(selfOffset) {
    
    let self = UtcOffset.fromString(selfOffset);
    
    let out = self.hoursPart;
    

    return out;
}
export function minutesPart(selfOffset) {
    
    let self = UtcOffset.fromString(selfOffset);
    
    let out = self.minutesPart;
    

    return out;
}
export function secondsPart(selfOffset) {
    
    let self = UtcOffset.fromString(selfOffset);
    
    let out = self.secondsPart;
    

    return out;
}
