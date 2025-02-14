import { IanaParserExtended } from "icu4x"
export function canonicalizeIana(value) {
    
    let ianaParserExtended = new IanaParserExtended();
    
    let out = ianaParserExtended.canonicalizeIana(value);
    

    return out;
}
export function canonicalIanaFromBcp47(value) {
    
    let ianaParserExtended = new IanaParserExtended();
    
    let out = ianaParserExtended.canonicalIanaFromBcp47(value);
    

    return out;
}
