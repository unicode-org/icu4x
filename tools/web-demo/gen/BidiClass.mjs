import { BidiClass } from "icu4x"
export function forChar(ch) {
    
    let out = BidiClass.forChar(ch);
    
    out = out?.value || 'None';;
    

    return out;
}
