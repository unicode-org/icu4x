import { SignedFixedDecimal } from "icu4x"
export function toString(signedFixedDecimalF, signedFixedDecimalMagnitude) {
    
    let signedFixedDecimal = SignedFixedDecimal.fromNumberWithLowerMagnitude(signedFixedDecimalF,signedFixedDecimalMagnitude);
    
    let out = signedFixedDecimal.toString();
    

    return out;
}
