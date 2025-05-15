import { CanonicalCombiningClassMap } from "icu4x"
export function get(ch) {
    
    let self = new CanonicalCombiningClassMap();
    
    let out = self.get(ch);
    

    return out;
}
