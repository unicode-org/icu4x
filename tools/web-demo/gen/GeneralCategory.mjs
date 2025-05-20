import { GeneralCategory } from "icu4x"
export function forChar(ch) {
    
    let out = GeneralCategory.forChar(ch);
    
    out = out?.value || 'None';;
    

    return out;
}
