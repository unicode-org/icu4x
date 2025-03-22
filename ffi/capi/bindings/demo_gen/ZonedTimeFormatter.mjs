import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeFormatter } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
import { ZonedTimeFormatter } from "icu4x"
export function format(zonedTimeFormatterLocaleName, zonedTimeFormatterFormatterLocaleName, zonedTimeFormatterFormatterLength, zonedTimeFormatterFormatterTimePrecision, zonedTimeFormatterFormatterAlignment, timeHour, timeMinute, timeSecond, timeSubsecond, zoneTimeZoneIdId, zoneOffsetOffset, zoneZoneVariant) {
    
    let zonedTimeFormatterLocale = Locale.fromString(zonedTimeFormatterLocaleName);
    
    let zonedTimeFormatterFormatterLocale = Locale.fromString(zonedTimeFormatterFormatterLocaleName);
    
    let zonedTimeFormatterFormatter = new TimeFormatter(zonedTimeFormatterFormatterLocale,zonedTimeFormatterFormatterLength,zonedTimeFormatterFormatterTimePrecision,zonedTimeFormatterFormatterAlignment);
    
    let zonedTimeFormatter = ZonedTimeFormatter.createGenericShort(zonedTimeFormatterLocale,zonedTimeFormatterFormatter);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeSubsecond);
    
    let zoneTimeZoneId = TimeZone.createFromBcp47(zoneTimeZoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneTimeZoneId,zoneOffset,zoneZoneVariant);
    
    let out = zonedTimeFormatter.format(time,zone);
    

    return out;
}
