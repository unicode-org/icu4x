import { Locale } from "icu4x"
import { Time } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
import { ZonedTimeFormatter } from "icu4x"
export function format(zonedTimeFormatterLocaleName, zonedTimeFormatterLength, zonedTimeFormatterTimePrecision, zonedTimeFormatterAlignment, timeHour, timeMinute, timeSecond, timeSubsecond, zoneTimeZoneIdId, zoneOffsetOffset, zoneZoneVariant) {
    
    let zonedTimeFormatterLocale = Locale.fromString(zonedTimeFormatterLocaleName);
    
    let zonedTimeFormatter = ZonedTimeFormatter.createGenericShort(zonedTimeFormatterLocale,zonedTimeFormatterLength,zonedTimeFormatterTimePrecision,zonedTimeFormatterAlignment);
    
    let time = new Time(timeHour,timeMinute,timeSecond,timeSubsecond);
    
    let zoneTimeZoneId = TimeZone.createFromBcp47(zoneTimeZoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneTimeZoneId,zoneOffset,zoneZoneVariant);
    
    let out = zonedTimeFormatter.format(time,zone);
    

    return out;
}
