import { GraphemeClusterBreak } from "icu4x"
export function forChar(ch) {
    
    let out = GraphemeClusterBreak.forChar(ch);
    
    out = out.value;;
    

    return out;
}
