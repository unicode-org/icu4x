import { TimeZoneVariant } from "icu4x"
export function fromRearguardIsdst(isdst) {
    
    let out = TimeZoneVariant.fromRearguardIsdst(isdst);
    
    out = out?.value || 'None';;
    

    return out;
}
