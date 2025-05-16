import { TimePrecision } from "icu4x"
export function fromSubsecondDigits(digits) {
    
    let out = TimePrecision.fromSubsecondDigits(digits);
    
    out = out?.value || 'None';;
    

    return out;
}
