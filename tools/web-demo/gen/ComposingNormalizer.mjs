import { ComposingNormalizer } from "icu4x"
export function normalize(s) {
    
    let self = ComposingNormalizer.createNfc();
    
    let out = self.normalize(s);
    

    return out;
}
export function isNormalized(s) {
    
    let self = ComposingNormalizer.createNfc();
    
    let out = self.isNormalized(s);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function isNormalizedUpTo(s) {
    
    let self = ComposingNormalizer.createNfc();
    
    let out = self.isNormalizedUpTo(s);
    

    return out;
}
