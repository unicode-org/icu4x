import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { DateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function format(dateTimeFormatterLocaleName, dateTimeFormatterLength, dateYear, dateMonth, dateDay, dateCalendarLocaleName, timeHour, timeMinute, timeSecond, timeNanosecond) {
    
    let dateTimeFormatterLocale = Locale.fromString(dateTimeFormatterLocaleName);
    
    let dateTimeFormatter = DateTimeFormatter.createWithLength(dateTimeFormatterLocale,dateTimeFormatterLength);
    
    let dateCalendarLocale = Locale.fromString(dateCalendarLocaleName);
    
    let dateCalendar = Calendar.createForLocale(dateCalendarLocale);
    
    let date = Date.fromIsoInCalendar(dateYear,dateMonth,dateDay,dateCalendar);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let out = dateTimeFormatter.format(date,time);
    

    return out;
}
export function formatIso(dateTimeFormatterLocaleName, dateTimeFormatterLength, dateYear, dateMonth, dateDay, timeHour, timeMinute, timeSecond, timeNanosecond) {
    
    let dateTimeFormatterLocale = Locale.fromString(dateTimeFormatterLocaleName);
    
    let dateTimeFormatter = DateTimeFormatter.createWithLength(dateTimeFormatterLocale,dateTimeFormatterLength);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let out = dateTimeFormatter.formatIso(date,time);
    

    return out;
}
