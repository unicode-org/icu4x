import { ExemplarCharacters } from "icu4x"
import { Locale } from "icu4x"
export function containsStr(selfLocaleName, s) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = ExemplarCharacters.createMain(selfLocale);
    
    let out = self.containsStr(s);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
export function contains(selfLocaleName, cp) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = ExemplarCharacters.createMain(selfLocale);
    
    let out = self.contains(cp);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
