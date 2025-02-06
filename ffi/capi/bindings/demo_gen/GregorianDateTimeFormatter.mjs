import { GregorianDateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
export function formatIso(gregorianDateTimeFormatterLocaleName, gregorianDateTimeFormatterLength, dateYear, dateMonth, dateDay, timeHour, timeMinute, timeSecond, timeNanosecond) {
    
    let gregorianDateTimeFormatterLocale = Locale.fromString(gregorianDateTimeFormatterLocaleName);
    
    let gregorianDateTimeFormatter = GregorianDateTimeFormatter.createWithLength(gregorianDateTimeFormatterLocale,gregorianDateTimeFormatterLength);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let out = gregorianDateTimeFormatter.formatIso(date,time);
    

    return out;
}
