import { IndicSyllabicCategory } from "icu4x"
export function forChar(ch) {
    
    let out = IndicSyllabicCategory.forChar(ch);
    
    out = out?.value || 'None';;
    

    return out;
}
