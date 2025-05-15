import { EastAsianWidth } from "icu4x"
export function forChar(ch) {
    
    let out = EastAsianWidth.forChar(ch);
    
    out = out.value;;
    

    return out;
}
