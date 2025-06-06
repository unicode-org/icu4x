import { Decimal } from "icu4x"
export function digitAt(selfV, magnitude) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.digitAt(magnitude);
    

    return out;
}
export function magnitudeStart(selfV) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.magnitudeStart;
    

    return out;
}
export function magnitudeEnd(selfV) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.magnitudeEnd;
    

    return out;
}
export function nonzeroMagnitudeStart(selfV) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.nonzeroMagnitudeStart;
    

    return out;
}
export function nonzeroMagnitudeEnd(selfV) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.nonzeroMagnitudeEnd;
    

    return out;
}
export function isZero(selfV) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.isZero;
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function sign(selfV) {
    
    let self = Decimal.fromString(selfV);
    
    let out = self.sign;
    
    out = out?.value || 'None';;
    

    return out;
}
