import { WordBreak } from "icu4x"
export function forChar(ch) {
    
    let out = WordBreak.forChar(ch);
    
    out = out.value;;
    

    return out;
}
