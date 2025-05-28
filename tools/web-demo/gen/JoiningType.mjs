import { JoiningType } from "icu4x"
export function forChar(ch) {
    
    let out = JoiningType.forChar(ch);
    
    out = out?.value || 'None';;
    

    return out;
}
