import { Decimal } from "icu4x"
export function toString(signedFixedDecimalF, signedFixedDecimalMagnitude) {
    
    let signedFixedDecimal = Decimal.fromNumberWithLowerMagnitude(signedFixedDecimalF,signedFixedDecimalMagnitude);
    
    let out = signedFixedDecimal.toString();
    

    return out;
}
