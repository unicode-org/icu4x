import { SentenceBreak } from "icu4x"
export function forChar(ch) {
    
    let out = SentenceBreak.forChar(ch);
    
    out = out.value;;
    

    return out;
}
