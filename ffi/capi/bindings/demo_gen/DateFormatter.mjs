import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { DateFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
export function format(dateFormatterLocaleName, dateFormatterLength, valueYear, valueMonth, valueDay, valueCalendarKind) {
    
    let dateFormatterLocale = Locale.fromString(dateFormatterLocaleName);
    
    let dateFormatter = DateFormatter.createWithLength(dateFormatterLocale,dateFormatterLength);
    
    let valueCalendar = new Calendar(valueCalendarKind);
    
    let value = Date.fromIsoInCalendar(valueYear,valueMonth,valueDay,valueCalendar);
    
    let out = dateFormatter.format(value);
    

    return out;
}
export function formatIso(dateFormatterLocaleName, dateFormatterLength, valueYear, valueMonth, valueDay) {
    
    let dateFormatterLocale = Locale.fromString(dateFormatterLocaleName);
    
    let dateFormatter = DateFormatter.createWithLength(dateFormatterLocale,dateFormatterLength);
    
    let value = new IsoDate(valueYear,valueMonth,valueDay);
    
    let out = dateFormatter.formatIso(value);
    

    return out;
}
