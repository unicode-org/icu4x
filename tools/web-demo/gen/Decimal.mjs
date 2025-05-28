import { Decimal } from "icu4x"
export function digitAt(selfF, magnitude) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.digitAt(magnitude);
    

    return out;
}
export function magnitudeStart(selfF) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.magnitudeStart;
    

    return out;
}
export function magnitudeEnd(selfF) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.magnitudeEnd;
    

    return out;
}
export function nonzeroMagnitudeStart(selfF) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.nonzeroMagnitudeStart;
    

    return out;
}
export function nonzeroMagnitudeEnd(selfF) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.nonzeroMagnitudeEnd;
    

    return out;
}
export function isZero(selfF) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.isZero;
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function sign(selfF) {
    
    let self = Decimal.fromNumberWithRoundTripPrecision(selfF);
    
    let out = self.sign;
    
    out = out?.value || 'None';;
    

    return out;
}
