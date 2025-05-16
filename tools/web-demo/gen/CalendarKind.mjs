import { CalendarKind } from "icu4x"
import { Locale } from "icu4x"
export function create(localeName) {
    
    let locale = Locale.fromString(localeName);
    
    let out = CalendarKind.create(locale);
    
    out = out?.value || 'None';;
    

    return out;
}
