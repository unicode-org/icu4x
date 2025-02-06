import { TimeZoneIdMapper } from "icu4x"
export function ianaToBcp47(value) {
    
    let timeZoneIdMapper = new TimeZoneIdMapper();
    
    let out = timeZoneIdMapper.ianaToBcp47(value);
    

    return out;
}
export function normalizeIana(value) {
    
    let timeZoneIdMapper = new TimeZoneIdMapper();
    
    let out = timeZoneIdMapper.normalizeIana(value);
    

    return out;
}
export function canonicalizeIana(value) {
    
    let timeZoneIdMapper = new TimeZoneIdMapper();
    
    let out = timeZoneIdMapper.canonicalizeIana(value);
    

    return out;
}
export function findCanonicalIanaFromBcp47(value) {
    
    let timeZoneIdMapper = new TimeZoneIdMapper();
    
    let out = timeZoneIdMapper.findCanonicalIanaFromBcp47(value);
    

    return out;
}
