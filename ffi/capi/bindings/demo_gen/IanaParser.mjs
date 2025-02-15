import { IanaParser } from "icu4x"
export function ianaToBcp47(value) {
    
    let ianaParser = new IanaParser();
    
    let out = ianaParser.ianaToBcp47(value);
    

    return out;
}
export function normalizeIana(value) {
    
    let ianaParser = new IanaParser();
    
    let out = ianaParser.normalizeIana(value);
    

    return out;
}
export function canonicalizeIana(value) {
    
    let ianaParser = new IanaParser();
    
    let out = ianaParser.canonicalizeIana(value);
    

    return out;
}
export function findCanonicalIanaFromBcp47(value) {
    
    let ianaParser = new IanaParser();
    
    let out = ianaParser.findCanonicalIanaFromBcp47(value);
    

    return out;
}
