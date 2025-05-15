import { Locale } from "icu4x"
import { WeekInformation } from "icu4x"
export function firstWeekday(selfLocaleName) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = new WeekInformation(selfLocale);
    
    let out = self.firstWeekday;
    
    out = out.value;;
    

    return out;
}
export function isWeekend(selfLocaleName, day) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = new WeekInformation(selfLocale);
    
    let out = self.isWeekend(day);
    
    out = out ? 'true' : 'false';;
    

    return out;
}
