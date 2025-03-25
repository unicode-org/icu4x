import { Locale } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneFormatter } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
export function format(timeZoneFormatterLocaleName, zoneTimeZoneIdId, zoneOffsetOffset, zoneZoneVariant) {
    
    let timeZoneFormatterLocale = Locale.fromString(timeZoneFormatterLocaleName);
    
    let timeZoneFormatter = TimeZoneFormatter.createGenericShort(timeZoneFormatterLocale);
    
    let zoneTimeZoneId = TimeZone.createFromBcp47(zoneTimeZoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneTimeZoneId,zoneOffset,zoneZoneVariant);
    
    let out = timeZoneFormatter.format(zone);
    

    return out;
}
