import { ListFormatter } from "icu4x"
import { Locale } from "icu4x"
export function format(selfLocaleName, selfLength, list) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = ListFormatter.createAndWithLength(selfLocale,selfLength);
    
    let out = self.format(list);
    

    return out;
}
