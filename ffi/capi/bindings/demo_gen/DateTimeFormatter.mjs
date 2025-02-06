import { DateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatIso(dateTimeFormatterLocaleName, dateTimeFormatterLength, dateTimeFormatterTimePrecision, dateTimeFormatterAlignment, dateTimeFormatterYearStyle, dateYear, dateMonth, dateDay, timeHour, timeMinute, timeSecond, timeNanosecond) {
    
    let dateTimeFormatterLocale = Locale.fromString(dateTimeFormatterLocaleName);
    
    let dateTimeFormatter = DateTimeFormatter.createYmdt(dateTimeFormatterLocale,dateTimeFormatterLength,dateTimeFormatterTimePrecision,dateTimeFormatterAlignment,dateTimeFormatterYearStyle);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let out = dateTimeFormatter.formatIso(date,time);
    

    return out;
}
