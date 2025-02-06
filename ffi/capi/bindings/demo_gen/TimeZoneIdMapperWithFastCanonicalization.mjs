import { TimeZoneIdMapperWithFastCanonicalization } from "icu4x"
export function canonicalizeIana(value) {
    
    let timeZoneIdMapperWithFastCanonicalization = new TimeZoneIdMapperWithFastCanonicalization();
    
    let out = timeZoneIdMapperWithFastCanonicalization.canonicalizeIana(value);
    

    return out;
}
export function canonicalIanaFromBcp47(value) {
    
    let timeZoneIdMapperWithFastCanonicalization = new TimeZoneIdMapperWithFastCanonicalization();
    
    let out = timeZoneIdMapperWithFastCanonicalization.canonicalIanaFromBcp47(value);
    

    return out;
}
