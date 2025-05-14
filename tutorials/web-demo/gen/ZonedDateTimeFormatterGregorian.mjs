import { DateTimeFormatterGregorian } from "icu4x"
import { IsoDate } from "icu4x"
import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
import { ZonedDateTimeFormatterGregorian } from "icu4x"
export function formatIso(selfLocaleName, selfFormatterLocaleName, selfFormatterLength, selfFormatterTimePrecision, selfFormatterAlignment, selfFormatterYearStyle, isoDateYear, isoDateMonth, isoDateDay, timeHour, timeMinute, timeSecond, timeSubsecond, zoneIdId, zoneOffsetOffset, zoneVariant) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let selfFormatterLocale = Locale.fromString(selfFormatterLocaleName);
    
    let selfFormatter = DateTimeFormatterGregorian.createYmdt(selfFormatterLocale,selfFormatterLength,selfFormatterTimePrecision,selfFormatterAlignment,selfFormatterYearStyle);
    
    let self = ZonedDateTimeFormatterGregorian.createGenericShort(selfLocale,selfFormatter);
    
    let isoDate = new IsoDate(isoDateYear,isoDateMonth,isoDateDay);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeSubsecond);
    
    let zoneId = TimeZone.createFromBcp47(zoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneId,zoneOffset,zoneVariant);
    
    let out = self.formatIso(isoDate,time,zone);
    

    return out;
}
