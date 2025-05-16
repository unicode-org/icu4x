import { PluralCategory } from "icu4x"
export function getForCldrString(s) {
    
    let out = PluralCategory.getForCldrString(s);
    
    out = out?.value || 'None';;
    

    return out;
}
