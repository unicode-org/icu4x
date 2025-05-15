import { Locale } from "icu4x"
import { TimeZone } from "icu4x"
import { TimeZoneFormatter } from "icu4x"
import { TimeZoneInfo } from "icu4x"
import { UtcOffset } from "icu4x"
export function format(selfLocaleName, zoneIdId, zoneOffsetOffset, zoneVariant) {
    
    let selfLocale = Locale.fromString(selfLocaleName);
    
    let self = TimeZoneFormatter.createGenericShort(selfLocale);
    
    let zoneId = TimeZone.createFromBcp47(zoneIdId);
    
    let zoneOffset = UtcOffset.fromString(zoneOffsetOffset);
    
    let zone = new TimeZoneInfo(zoneId,zoneOffset,zoneVariant);
    
    let out = self.format(zone);
    

    return out;
}
