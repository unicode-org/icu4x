import { LineBreak } from "icu4x"
export function forChar(ch) {
    
    let out = LineBreak.forChar(ch);
    
    out = out?.value || 'None';;
    

    return out;
}
