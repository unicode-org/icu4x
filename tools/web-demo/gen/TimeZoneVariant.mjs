import { TimeZoneVariant } from "icu4x"
export function fromRearguardIsdst(self, isdst) {
    
    let out = self.fromRearguardIsdst(isdst);
    
    out = out?.value || 'None';;
    

    return out;
}
