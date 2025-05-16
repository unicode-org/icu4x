import { Locale } from "icu4x"
import { LocaleDirectionality } from "icu4x"
export function get(localeName) {
    
    let self = new LocaleDirectionality();
    
    let locale = Locale.fromString(localeName);
    
    let out = self.get(locale);
    
    out = out?.value || 'None';;
    

    return out;
}
