import { Locale } from "icu4x"
export function basename(selfName) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.basename;
    

    return out;
}
export function getUnicodeExtension(selfName, s) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.getUnicodeExtension(s);
    

    return out;
}
export function language(selfName) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.language;
    

    return out;
}
export function region(selfName) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.region;
    

    return out;
}
export function script(selfName) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.script;
    

    return out;
}
export function normalize(s) {
    
    let out = Locale.normalize(s);
    

    return out;
}
export function toString(selfName) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.toString();
    

    return out;
}
export function normalizingEq(selfName, other) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.normalizingEq(other);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function compareToString(selfName, other) {
    
    let self = Locale.fromString(selfName);
    
    let out = self.compareToString(other);
    
    out = out == 0 ? '==' : out == 1 ? '>' : '<';;
    

    return out;
}
export function compareTo(selfName, otherName) {
    
    let self = Locale.fromString(selfName);
    
    let other = Locale.fromString(otherName);
    
    let out = self.compareTo(other);
    
    out = out == 0 ? '==' : out == 1 ? '>' : '<';;
    

    return out;
}
