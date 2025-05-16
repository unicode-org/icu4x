import { Script } from "icu4x"
export function forChar(ch) {
    
    let out = Script.forChar(ch);
    
    out = out?.value || 'None';;
    

    return out;
}
