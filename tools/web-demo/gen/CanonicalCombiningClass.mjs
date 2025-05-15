import { CanonicalCombiningClass } from "icu4x"
export function forChar(ch) {
    
    let out = CanonicalCombiningClass.forChar(ch);
    
    out = out.value;;
    

    return out;
}
