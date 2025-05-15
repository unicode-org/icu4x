import { DecomposingNormalizer } from "icu4x"
export function normalize(s) {
    
    let self = DecomposingNormalizer.createNfd();
    
    let out = self.normalize(s);
    

    return out;
}
export function isNormalized(s) {
    
    let self = DecomposingNormalizer.createNfd();
    
    let out = self.isNormalized(s);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function isNormalizedUpTo(s) {
    
    let self = DecomposingNormalizer.createNfd();
    
    let out = self.isNormalizedUpTo(s);
    

    return out;
}
