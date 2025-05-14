import { TimeZone } from "icu4x"
export function isUnknown(selfId) {
    
    let self = TimeZone.createFromBcp47(selfId);
    
    let out = self.isUnknown();
    
    out = out ? 'true' : 'false';;
    

    return out;
}
