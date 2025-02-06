import { Calendar } from "icu4x"
import { Date } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { ZonedDateTimeFormatter } from "icu4x"
export function format(zonedDateTimeFormatterLocaleName, zonedDateTimeFormatterLength, dateYear, dateMonth, dateDay, dateCalendarLocaleName, timeHour, timeMinute, timeSecond, timeNanosecond, zoneBcp47Id, zoneOffsetSeconds, zoneDst) {
    
    let zonedDateTimeFormatterLocale = Locale.fromString(zonedDateTimeFormatterLocaleName);
    
    let zonedDateTimeFormatter = ZonedDateTimeFormatter.createWithLength(zonedDateTimeFormatterLocale,zonedDateTimeFormatterLength);
    
    let dateCalendarLocale = Locale.fromString(dateCalendarLocaleName);
    
    let dateCalendar = Calendar.createForLocale(dateCalendarLocale);
    
    let date = Date.fromIsoInCalendar(dateYear,dateMonth,dateDay,dateCalendar);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let zone = new TimeZoneInfo(zoneBcp47Id,zoneOffsetSeconds,zoneDst);
    
    let out = zonedDateTimeFormatter.format(date,time,zone);
    

    return out;
}
export function formatIso(zonedDateTimeFormatterLocaleName, zonedDateTimeFormatterLength, dateYear, dateMonth, dateDay, timeHour, timeMinute, timeSecond, timeNanosecond, zoneBcp47Id, zoneOffsetSeconds, zoneDst) {
    
    let zonedDateTimeFormatterLocale = Locale.fromString(zonedDateTimeFormatterLocaleName);
    
    let zonedDateTimeFormatter = ZonedDateTimeFormatter.createWithLength(zonedDateTimeFormatterLocale,zonedDateTimeFormatterLength);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let zone = new TimeZoneInfo(zoneBcp47Id,zoneOffsetSeconds,zoneDst);
    
    let out = zonedDateTimeFormatter.formatIso(date,time,zone);
    

    return out;
}
