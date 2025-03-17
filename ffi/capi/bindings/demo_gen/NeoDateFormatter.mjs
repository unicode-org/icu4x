import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { NeoDateFormatter } from "icu4x"
export function formatIso(neoDateFormatterLocaleName, neoDateFormatterLength, neoDateFormatterAlignment, neoDateFormatterYearStyle, dateYear, dateMonth, dateDay) {
    
    let neoDateFormatterLocale = Locale.fromString(neoDateFormatterLocaleName);
    
    let neoDateFormatter = NeoDateFormatter.createYmd(neoDateFormatterLocale,neoDateFormatterLength,neoDateFormatterAlignment,neoDateFormatterYearStyle);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let out = neoDateFormatter.formatIso(date);
    

    return out;
}
export function formatSameCalendar(neoDateFormatterLocaleName, neoDateFormatterLength, neoDateFormatterAlignment, neoDateFormatterYearStyle, dateYear, dateMonth, dateDay, dateCalendarLocaleName) {
    
    let neoDateFormatterLocale = Locale.fromString(neoDateFormatterLocaleName);
    
    let neoDateFormatter = NeoDateFormatter.createYmd(neoDateFormatterLocale,neoDateFormatterLength,neoDateFormatterAlignment,neoDateFormatterYearStyle);
    
    let dateCalendarLocale = Locale.fromString(dateCalendarLocaleName);
    
    let dateCalendar = Calendar.createForLocale(dateCalendarLocale);
    
    let date = Date.fromIsoInCalendar(dateYear,dateMonth,dateDay,dateCalendar);
    
    let out = neoDateFormatter.formatSameCalendar(date);
    

    return out;
}
