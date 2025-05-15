import { HangulSyllableType } from "icu4x"
export function forChar(ch) {
    
    let out = HangulSyllableType.forChar(ch);
    
    out = out.value;;
    

    return out;
}
