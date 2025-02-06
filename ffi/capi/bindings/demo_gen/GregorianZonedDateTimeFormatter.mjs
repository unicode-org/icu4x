import { GregorianZonedDateTimeFormatter } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeZoneInfo } from "icu4x"
export function formatIso(gregorianZonedDateTimeFormatterLocaleName, gregorianZonedDateTimeFormatterLength, dateYear, dateMonth, dateDay, timeHour, timeMinute, timeSecond, timeNanosecond, zoneBcp47Id, zoneOffsetSeconds, zoneDst) {
    
    let gregorianZonedDateTimeFormatterLocale = Locale.fromString(gregorianZonedDateTimeFormatterLocaleName);
    
    let gregorianZonedDateTimeFormatter = GregorianZonedDateTimeFormatter.createWithLength(gregorianZonedDateTimeFormatterLocale,gregorianZonedDateTimeFormatterLength);
    
    let date = new IsoDate(dateYear,dateMonth,dateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeNanosecond);
    
    let zone = new TimeZoneInfo(zoneBcp47Id,zoneOffsetSeconds,zoneDst);
    
    let out = gregorianZonedDateTimeFormatter.formatIso(date,time,zone);
    

    return out;
}
